use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;

// Abstracts for the api
#[derive(Default, Clone)]
pub struct LoginDetails {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub accessToken: String,
}
pub async fn api_login(username: String, password: String) -> LoginResponse {
    let body = json!({
        "email" : username,
        "password" : password
    });
    let response = Request::post("/api/v1/login")
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<LoginResponse>()
        .await
        .unwrap();
    response
}
