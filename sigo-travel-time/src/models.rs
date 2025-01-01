// sigo-travel-time/src/models.rs
// 웹 서비스 데이터 모델 Rust 구조체 정의 (DTO)

use actix_web::web;
// use chrono::NaiveDateTime; // 등록 시간 기록하려고 쓰는 서드파티 크레이트
use serde::{Deserialize, Serialize}; // Rust 데이터 구조 - HTTP msg 전송용 포맷 역직렬화, 직렬화

// TMAP 대중교통 API 인풋 구조체
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TMAPtransitAPIInput {
    pub startX: String,
    pub startY: String,
    pub endX: String,
    pub endY: String,
}

// HTTP request 데이터를 Rust 구조체로 변환.
impl From<web::Json<TMAPtransitAPIInput>> for TMAPtransitAPIInput {
    fn from(course: web::Json<TMAPtransitAPIInput>) -> Self {
        TMAPtransitAPIInput {
            startX: startX.clone(),
            startY: startY.clone(),
            endX: endX.clone(),
            endY: endY.clone(),
        }
    }
}
