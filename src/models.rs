use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CldResponse {
    pub public_id: String,
    pub secure_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct APIResponse {
    pub status: u16,
    pub data: CldResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct URLFile {
    pub file_path: String,
}
