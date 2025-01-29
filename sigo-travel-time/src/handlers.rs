// sigo-travel-time/src/handlers.rs
// 각 HTTP 요청을 수행하는 핸들러 함수들을 작성 (서비스)

use super::models::{RequestBody, TMAPAPIInput, Transport};
use super::state::AppState;
use crate::models::{TmapDrivingResponse, TmapTransitResponse, TmapWalkingResponse};
use actix_web::{web, HttpResponse};
use chrono::prelude::*;
use serde_json::Value;
use std::env;

pub async fn time_comparison(
    req: web::Json<RequestBody>, // Actix 웹이 request body를 자동으로 JSON으로 파싱해서 TMAPAPIInput 구조체로 변환한다.
    app_state: web::Data<AppState>,
    query: web::Query<Transport>,
) -> HttpResponse {
    // TMAP API 호출해서 total_time 값 받아오기
    let tmap_api_input = TMAPAPIInput::new(
        req.start_x.clone(),
        req.start_y.clone(),
        req.end_x.clone(),
        req.end_y.clone(),
    );
    let total_time_res = _get_total_time(tmap_api_input, app_state, query).await;
    let total_time_sec = match total_time_res {
        Ok(sec) => sec, // Ok -> sec의 i64 값을 total_time_sec으로 저장
        Err(http_response) => return http_response, // 에러 발생할 시 즉시 반환
    };
    let preparation_time_sec: i64 = req.preparation_time * 60; // 요청 본문에서 받아오기 -> 초 단위로 변환

    // NaiveDateTime 생성
    let naive_time = NaiveDateTime::parse_from_str(&req.alarm_time, "%Y-%m-%d %H:%M:%S").unwrap();
    // NaiveDateTime을 DateTime<Utc>로 변환
    let alarm_time: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_time, Utc);
    let now: DateTime<Utc> = Utc::now();

    let required_time = total_time_sec + preparation_time_sec;
    let remaining_time = alarm_time.timestamp() - (now.timestamp() + 9 * 3600); // now 값에 GMT+9 시간대로 시간 보정

    let should_alarm_ring;

    println!("total_time: {}초", total_time_sec);
    println!("준비시간: {}초", preparation_time_sec);
    println!("알람 시간: {:?}", alarm_time);
    println!("현재 시간: {:?}", now);
    println!();
    println!(
        "required_time: {:?}, remaining_time: {:?}",
        required_time, remaining_time
    );
    println!();

    if remaining_time <= required_time {
        should_alarm_ring = true;
    } else if remaining_time > required_time && remaining_time - required_time <= 300 {
        should_alarm_ring = true;
    } else {
        should_alarm_ring = false;
    }

    let res_json = serde_json::json!(
        {
            "should_alarm_ring": should_alarm_ring
        }
    );

    if should_alarm_ring {
        HttpResponse::Ok().json(res_json)
    } else {
        HttpResponse::Accepted().json(res_json)
    }
}

async fn _get_total_time(
    api_input_info: TMAPAPIInput,
    app_state: web::Data<AppState>,
    query: web::Query<Transport>,
) -> Result<i64, HttpResponse> {
    let transport = query.transport.clone();

    match transport.as_str() {
        "driving" => {
            let response = _get_travel_time_by_driving(api_input_info, app_state).await?;
            Ok(response.features[0].properties.total_time as i64)
        }
        "transit" => {
            let response = _get_travel_time_by_transit(api_input_info, app_state).await?;
            let total_time = response.get_total_time().unwrap_or(0);
            Ok(total_time)
        }
        "walking" => {
            let response = _get_travel_time_by_walking(api_input_info, app_state).await?;
            let total_time = response.get_total_time().unwrap_or(0); // 기본값 0
            Ok(total_time)
        }
        _ => Err(HttpResponse::BadRequest().body("Query 파라미터가 잘못되었습니다.")),
    }
}

// 대중교통을 이용했을 때 걸리는 시간을 받아오는 기능
// TMAP 대중교통 API 활용
async fn _get_travel_time_by_transit(
    api_input_info: TMAPAPIInput,
    app_state: web::Data<AppState>,
) -> Result<TmapTransitResponse, HttpResponse> {
    let http_client = &app_state.http_client;
    let tmap_api_endpoint = "https://apis.openapi.sk.com/transit/routes";
    let app_key = env::var("TMAP_API_KEY").expect("Failed to get TMAP_API_KEY in .env");

    let request_body = serde_json::json!({
        "startX": api_input_info.start_x,
        "startY": api_input_info.start_y,
        "endX": api_input_info.end_x,
        "endY": api_input_info.end_y,
        "count" : 1,
        "lang": 0,
        "format":"json"
    });

    match http_client
        .post(tmap_api_endpoint)
        .header("content-type", "application/json")
        .header("appKey", app_key)
        .header("accept", "application/json")
        .json(&request_body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let json_response: Value = response.json().await.map_err(|_| {
                    HttpResponse::InternalServerError().body("Failed to parse response")
                })?;
                let tmap_transit_response =
                    TmapTransitResponse::from_json(json_response).map_err(|_| {
                        HttpResponse::InternalServerError()
                            .body("Failed to parse TmapTransitResponse")
                    })?;
                Ok(tmap_transit_response)
            } else {
                Err(HttpResponse::BadRequest().body("Failed to fetch travel time from TMAP API"))
            }
        }
        Err(_) => Err(HttpResponse::InternalServerError().body("Failed to connect to TMAP API")),
    }
}

async fn _get_travel_time_by_driving(
    api_input_info: TMAPAPIInput,
    app_state: web::Data<AppState>,
) -> Result<TmapDrivingResponse, HttpResponse> {
    let http_client = &app_state.http_client;
    let app_key = env::var("TMAP_API_KEY").expect("Failed to get TMAP_API_KEY in .env");
    let tmap_api_endpoint = "https://apis.openapi.sk.com/tmap/routes?version=1";

    // https://openapi.sk.com/products/detail?svcSeq=4&menuSeq=46#Body_Parameters
    // 이 링크에서 각 Body Parameters 확인 !!
    let request_body = serde_json::json!({
        "startX": api_input_info.start_x,
        "startY": api_input_info.start_y,
        "endX": api_input_info.end_x,
        "endY": api_input_info.end_y,
        "searchOption" : 0, // 경로 탐색 옵션, 0(기본값): 교통최적 + 추천
        "totalValue": 2
    });

    match http_client
        .post(tmap_api_endpoint)
        .header("content-type", "application/json")
        .header("appKey", app_key)
        .header("accept", "application/json")
        .json(&request_body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let json_response: Value = response.json().await.map_err(|_| {
                    HttpResponse::InternalServerError().body("Failed to parse response")
                })?;
                let tmap_driving_response =
                    TmapDrivingResponse::from_json(json_response).map_err(|_| {
                        HttpResponse::InternalServerError()
                            .body("Failed to parse TmapDrivingResponse")
                    })?;
                Ok(tmap_driving_response)
            } else {
                Err(HttpResponse::BadRequest().body("Failed to fetch travel time from TMAP API"))
            }
        }
        Err(_) => Err(HttpResponse::InternalServerError().body("Failed to connect to TMAP API")),
    }
}

async fn _get_travel_time_by_walking(
    api_input_info: TMAPAPIInput,
    app_state: web::Data<AppState>,
) -> Result<TmapWalkingResponse, HttpResponse> {
    let http_client = &app_state.http_client;
    let app_key = env::var("TMAP_API_KEY").expect("Failed to get TMAP_API_KEY in .env");
    let tmap_api_endpoint = "https://apis.openapi.sk.com/tmap/routes/pedestrian?version=1";

    // https://openapi.sk.com/products/detail?svcSeq=4&menuSeq=45
    // 이 링크에서 각 Body Parameters 확인 !!
    // TODO: startName, endName 파라미터 또한 api 요청 body로 받아와야 할 듯
    let request_body = serde_json::json!({
        "startX": api_input_info.start_x,
        "startY": api_input_info.start_y,
        "startName": "출발지",
        "endX": api_input_info.end_x,
        "endY": api_input_info.end_y,
        "endName": "도착지"
    });

    match http_client
        .post(tmap_api_endpoint)
        .header("content-type", "application/json")
        .header("appKey", app_key)
        .header("accept", "application/json")
        .json(&request_body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                let json_response: Value = response.json().await.map_err(|_| {
                    HttpResponse::InternalServerError().body("Failed to parse response")
                })?;
                let tmap_walking_response =
                    TmapWalkingResponse::from_json(json_response).map_err(|_| {
                        HttpResponse::InternalServerError()
                            .body("Failed to parse TmapWalkingResponse")
                    })?;
                Ok(tmap_walking_response)
            } else {
                Err(HttpResponse::BadRequest().body("Failed to fetch travel time from TMAP API"))
            }
        }
        Err(_) => Err(HttpResponse::InternalServerError().body("Failed to connect to TMAP API")),
    }
}

pub async fn test_handler() -> HttpResponse {
    let should_alarm_ring = true;
    let res_json = serde_json::json!({
        "should_alarm_ring": should_alarm_ring
    });

    HttpResponse::Ok().json(res_json)
}
