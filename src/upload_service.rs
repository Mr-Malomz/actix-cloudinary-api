use std::env;

use actix_easy_multipart::{File, FromMultipart};
use dotenv::dotenv;
use reqwest::{blocking::Client, header};

#[derive(FromMultipart)]
struct Upload {
    file: File,
}

struct CloudinaryDetails {
    url: String,
}

impl Upload {
    fn url_helper() -> String {
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

        url
    }

    async fn cloudinary_helper(data: &Self) {
        let client = Client::new();
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        // let body = format!("{}&upload_preset", data.file.filename);
        let response = client
            .post(Upload::url_helper())
            .headers(headers)
            .body(data.file.)
            .send();
    }
}
