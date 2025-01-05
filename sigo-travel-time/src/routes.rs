// sigo-travel-time/src/routes.rs
// 각 route를 정의해두는 파일 (컨트롤러)

use super::handlers::*;
use actix_web::web;

pub fn travel_time_routes(cfg: &mut web::ServiceConfig) {
    let api_path = "/api/v0/travel-time";
    cfg.service(
        web::scope(&api_path)
            .route("/transit", web::post().to(get_travel_time_by_transit))
            .route("/driving", web::post().to(get_travel_time_by_driving))
            .route("/walking", web::post().to(get_travel_time_by_walking)),
    );
    cfg.service(web::scope("/test").route("/", web::get().to(test_handler)));
}
