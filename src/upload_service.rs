use std::{collections::HashMap, env, error::Error};

use actix_easy_multipart::{File, FromMultipart};
use actix_web::http;
use dotenv::dotenv;
use reqwest::{ Client};
use serde::Serialize;

use crate::models::{APIResponse, CldResponse, URLFile};

trait UploadHelper {}

async fn cloudinary_helper(file_part: String) -> Result<CldResponse, Box<dyn Error>> {
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

    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("upload_preset", &upload_preset);
    map.insert("file", &file_part);

    //making api request
    let client = Client::new();

    let resp2 = client
    .post(&url)
    .json(&map)
    .send()
    .await?;

        
    println!("{:#?}", resp2);

    let response = client
        .post(&url)
        .json(&map)
        .send()
        .await?
        .json::<CldResponse>()
        .await?;
    

    Ok(response)
}

#[derive(FromMultipart)]
struct UploadFromFile {
    file: File,
}

impl UploadFromFile {
    // fn upload(&self, cld: &impl UploadHelper) {
    //     // cld.cloudinary_helper("file_part".to_string())
    // }
}

#[derive(Serialize)]
pub struct UploadFromURL {}

impl UploadFromURL {
    pub async fn upload(&self, new_file: URLFile) -> Result<APIResponse, Box<dyn Error>> {
        let cld_upload = cloudinary_helper(new_file.file_path).await?;

        let json_data: APIResponse = APIResponse {
            status: http::StatusCode::OK.as_u16(),
            data: cld_upload,
        };

        Ok(json_data)
    }
}
