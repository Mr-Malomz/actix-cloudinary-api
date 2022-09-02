use std::env;

use actix_easy_multipart::{File, FromMultipart};
use dotenv::dotenv;
use reqwest::blocking::{multipart, Client};

fn cloudinary_helper(file_part: String) {
    //load data from env variable
    dotenv().ok();
    let cloud_name = match env::var("CLOUD_NAME") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };
    let upload_preset = match env::var("UPLOAD_PRESET") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable"),
    };

    let url = format!(
        "https://api.cloudinary.com/v1_1/{x}/image/upload",
        x = cloud_name
    );

    //header
    let form_data = multipart::Form::new()
        .text("upload_preset", upload_preset)
        .file("file", file_part)
        .unwrap();

    //making api request
    let client = Client::new();
    let response = client.post(url).multipart(form_data).send().unwrap();
}

#[derive(FromMultipart)]
struct UploadFromFile {
    file: File,
}

impl UploadFromFile {
    fn upload(&self) {}
}
