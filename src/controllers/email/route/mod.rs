use axum::{Router, routing::post};
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use crate::controllers::email::data::Email;

pub async fn create_user(Json(email): Json<Email>) -> Result<Json<Email>, impl IntoResponse> {
    let json_value = serde_json::to_value(&email).unwrap();
    let email_struct: Email = serde_json::from_value(json_value).unwrap();

    if !email_struct.email.contains("@") {
        return Err((
            StatusCode::BAD_REQUEST,
            "Failed to process email".to_string(),
        ));
    }

    println!("{:?}", email);
    Ok(Json(email))
}

pub fn routes() -> Router {
    Router::new().route("/create-user", post(create_user))
}