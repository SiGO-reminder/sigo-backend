// sigo-travel-time/src/models.rs
// 웹 서비스 데이터 모델 Rust 구조체 정의 (DTO)

use actix_web::web;
// use chrono::NaiveDateTime; // 등록 시간 기록하려고 쓰는 서드파티 크레이트
use serde::{Deserialize, Serialize}; // Rust 데이터 구조 - HTTP msg 전송용 포맷 역직렬화, 직렬화

// TMAP 대중교통 API 인풋 구조체
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TMAPtransitAPIInput {
    pub start_x: String,
    pub start_y: String,
    pub end_x: String,
    pub end_y: String,
}

// HTTP request 데이터를 Rust 구조체로 변환.
impl From<web::Json<TMAPtransitAPIInput>> for TMAPtransitAPIInput {
    fn from(req: web::Json<TMAPtransitAPIInput>) -> Self {
        TMAPtransitAPIInput {
            start_x: req.start_x.clone(),
            start_y: req.start_y.clone(),
            end_x: req.end_x.clone(),
            end_y: req.end_y.clone(),
        }
    }
}

// TMAP API (자동차 & 보행자 경로안내) API input 구조체
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TMAPAPIInput {
    pub start_x: String,
    pub start_y: String,
    pub start_name: String,
    pub end_x: String,
    pub end_y: String,
    pub end_name: String,
}

// HTTP request 데이터를 Rust 구조체로 변환.
impl From<web::Json<TMAPAPIInput>> for TMAPAPIInput {
    fn from(req: web::Json<TMAPAPIInput>) -> Self {
        TMAPAPIInput {
            start_x: req.start_x.clone(),
            start_y: req.start_y.clone(),
            start_name: req.start_name.clone(),
            end_x: req.end_x.clone(),
            end_y: req.end_y.clone(),
            end_name: req.end_name.clone(),
        }
    }
}
