// sigo-travel-time/src/handlers.rs
// 각 HTTP 요청을 수행하는 핸들러 함수들을 작성 (서비스)

use super::models::TMAPtransitAPIInput;
use super::state::AppState;
use actix_web::{web, HttpResponse};
// use chrono::Utc; // 등록 시간.

// 대중교통을 이용했을 때 걸리는 시간을 받아오는 기능
// TMAP 대중교통 API 활용
pub async fn get_travel_time_by_transit(
    api_input_info: web::Json<TMAPtransitAPIInput>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    // TMAP 대중교통 API call
    // api_input_info 속 데이터 활용
    HttpResponse::Ok().json("Added course")
}
