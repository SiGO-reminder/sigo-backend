// sigo-travel-time/src/handlers.rs
// 각 HTTP 요청을 수행하는 핸들러 함수들을 작성 (서비스)

use super::models::TMAPtransitAPIInput;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use serde_json::Value;
use std::env;
// use chrono::Utc; // 등록 시간.

// 대중교통을 이용했을 때 걸리는 시간을 받아오는 기능
// TMAP 대중교통 API 활용
pub async fn get_travel_time_by_transit(
    api_input_info: web::Json<TMAPtransitAPIInput>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
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
                // 응답을 JSON으로 변환
                match response.json::<Value>().await {
                    Ok(json_response) => HttpResponse::Ok().json(json_response),
                    Err(_) => HttpResponse::InternalServerError().body("Failed to parse response"),
                }
            } else {
                // TMAP API가 실패 상태 코드를 반환
                HttpResponse::BadRequest().body("Failed to fetch travel time from TMAP API")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to connect to TMAP API"),
    }
}

pub async fn get_travel_time_by_driving(
    api_input_info: web::Json<TMAPtransitAPIInput>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let http_client = &app_state.http_client;
    let tmap_api_endpoint = "https://apis.openapi.sk.com/tmap/routes?version=1";
    let app_key = env::var("TMAP_API_KEY").expect("Failed to get TMAP_API_KEY in .env");

    let request_body = serde_json::json!({
        "startX": api_input_info.start_x,
        "startY": api_input_info.start_y,
        "endX": api_input_info.end_x,
        "endY": api_input_info.end_y,
        "totalValue": 2 
    });

    match http_client
        .post(tmap_api_endpoint)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("appKey", app_key)
        .header("accept", "application/json")
        .header("Accept-Language", "ko")
        .json(&request_body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                // 응답을 JSON으로 변환
                match response.json::<Value>().await {
                    Ok(json_response) => HttpResponse::Ok().json(json_response),
                    Err(_) => HttpResponse::InternalServerError().body("Failed to parse response"),
                }
            } else {
                // TMAP API가 실패 상태 코드를 반환
                HttpResponse::BadRequest().body("Failed to fetch travel time from TMAP API")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to connect to TMAP API"),
    }
}

pub async fn test_handler() -> HttpResponse {
    HttpResponse::Ok().body("Test handler called")
}
