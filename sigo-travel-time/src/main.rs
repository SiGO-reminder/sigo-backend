use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

#[path = "handlers.rs"]
mod handlers;
#[path = "models.rs"]
mod models;
#[path = "routes.rs"]
mod routes;
#[path = "state.rs"]
mod state;

use routes::*; 
use state::AppState;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 애플리케이션 상태 초기화
    let shared_data = web::Data::new(AppState {
    });

    // 웹 애플리케이션 정의 closure
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(travel_time_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
