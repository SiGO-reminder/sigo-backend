// sigo-travel-time/src/routes.rs
// 각 route를 정의해두는 파일 (컨트롤러)

use super::handlers::*;
use actix_web::web;

pub fn travel_time_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/travel-time")
            .route("/", web::post().to(get_travel_time_by_transit)),
    );
}
