use actix_web::{HttpServer, App};

mod upload_routes;
mod upload_service;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new())
    .bind(("localhost", 8080))?
    .run()
    .await
}
