// sigo-travel-time/src/routes.rs
// 각 route를 정의해두는 파일 (컨트롤러)

use super::handlers::*;
use actix_web::web;

pub fn travel_time_routes(cfg: &mut web::ServiceConfig) {
    let api_path = "/api/v0/travel-time";
    cfg.service(
        web::scope(&api_path)
            .route("", web::post().to(get_travel_time))
    );
    cfg.service(web::scope("/test").route("", web::get().to(test_handler)));
}
