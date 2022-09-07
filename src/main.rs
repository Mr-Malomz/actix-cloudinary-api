use actix_web::{web::Data, App, HttpServer};
use upload_routes::{file_upload, url_upload};
use upload_service::UploadFromURL;

mod models;
mod upload_routes;
mod upload_service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let url_data = UploadFromURL {};
    let db_data_url = Data::new(url_data);
    HttpServer::new(move || {
        App::new()
            .app_data(db_data_url.clone())
            .service(file_upload)
            .service(url_upload)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
