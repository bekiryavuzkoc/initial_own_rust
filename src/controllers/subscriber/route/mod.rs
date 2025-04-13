use axum::routing::get;
use axum::{Router, routing::post};
use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::controllers::subscriber::data::Subscriber;
use crate::appstate::EmailState;


pub async fn create_user(State(state): State<Arc<Mutex<EmailState>>>, Json(email): Json<Subscriber>) -> Result<Json<Subscriber>, impl IntoResponse> {
    let json_value = serde_json::to_value(&email).unwrap();
    let email_struct: Subscriber = serde_json::from_value(json_value).unwrap();

    if !email_struct.email.contains("@") {
        return Err((
            StatusCode::BAD_REQUEST,
            "Failed to process email".to_string(),
        ));
    }
   
    let mut locked_state = state.lock().await;
    locked_state.add_new_email_list(email.email.clone());

    println!("{:?}", chrono::Utc::now());
    println!("{:?}", email.email.clone());
    Ok(Json(email))
}

pub async fn get_all_email_list(State(state): State<Arc<Mutex<EmailState>>>) -> Result<Json<Vec<String>>, impl IntoResponse> {
    let locked_state = state.lock().await;
    let vec = locked_state.get_all_email_list();
   
    if vec.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Email list is empty".to_string(),
        ));
    }
   
    Ok(Json(vec.to_vec()))
}

pub fn routes(state: Arc<Mutex<EmailState>>) -> Router {
    Router::new()
        .route("/create-user", post(create_user))
        .route("/subscribers", get(get_all_email_list))
        .with_state(state)
}