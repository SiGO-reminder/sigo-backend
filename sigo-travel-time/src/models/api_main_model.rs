// sigo-travel-time/src/models.rs
// 웹 서비스 데이터 모델 Rust 구조체 정의 (DTO)

use actix_web::web;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RequestBody {
    pub start_x: String,
    pub start_y: String,
    pub end_x: String,
    pub end_y: String,
    pub alarm_time: DateTime<Utc>,
    pub preparation_time: i64,
}

// TMAP API (자동차 & 보행자 경로안내) API input 구조체
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TMAPAPIInput {
    pub start_x: String,
    pub start_y: String,
    pub end_x: String,
    pub end_y: String,
}

// HTTP request 데이터를 Rust 구조체로 변환.
impl From<web::Json<TMAPAPIInput>> for TMAPAPIInput {
    fn from(req: web::Json<TMAPAPIInput>) -> Self {
        TMAPAPIInput {
            start_x: req.start_x.clone(),
            start_y: req.start_y.clone(),
            end_x: req.end_x.clone(),
            end_y: req.end_y.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct Transport {
    pub transport: String,
}
