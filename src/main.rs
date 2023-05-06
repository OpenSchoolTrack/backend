use actix_web::{App, HttpServer};

mod api;

use crate::api::v1::students::config as student_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(student_config))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
