use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use reqwest::Client;

#[path = "handlers.rs"]
mod handlers;
pub mod models; // 모델을 디렉토리로 변경해서 관리
#[path = "routes.rs"]
mod routes;
#[path = "state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let http_client = Client::new();
    let shared_data = web::Data::new(AppState { http_client }); // 애플리케이션 상태 초기화
    dotenv().ok(); // .env 파일에서 환경변수 로드

    // 웹 애플리케이션 정의 closure
    let app = move || {
        App::new()
            .app_data(shared_data.clone()) // 상태 등록
            .wrap(Cors::permissive().allowed_origin("http://127.0.0.1:8080")) // CORS 설정
            .configure(travel_time_routes)
    };

    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
