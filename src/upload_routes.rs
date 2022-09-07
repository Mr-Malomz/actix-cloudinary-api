use actix_easy_multipart::{extractor::MultipartForm, File, FromMultipart};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};

use crate::{models::URLFile, upload_service::UploadFromURL};

#[derive(FromMultipart)]
struct Upload {
    file: File,
}

#[post("/fileupload")]
pub async fn file_upload(form: MultipartForm<Upload>) -> impl Responder {
    format!("Received image of size: {}", form.file.size)
}

#[post("/urlupload")]
pub async fn url_upload(logic: Data<UploadFromURL>, data: Json<URLFile>) -> HttpResponse {
    let new_upload = URLFile {
        file_path: data.file_path.clone(),
    };
    let upload_details = logic.upload(new_upload).await;

    match upload_details {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
