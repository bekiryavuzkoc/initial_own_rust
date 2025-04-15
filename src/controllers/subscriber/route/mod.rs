use axum::extract::Path;
use axum::routing::get;
use axum::{Router, routing::post};
use axum::{
    extract::{State, Json},
    http::StatusCode,
    routing::delete,
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::controllers::subscriber::data::Subscriber;
use crate::appstate::SubscriberAppState;

use super::data::Email;


pub async fn subscribe(State(state): State<Arc<Mutex<SubscriberAppState>>>, Json(email): Json<Email>) -> Result<Json<String>, impl IntoResponse> {
    let json_value = serde_json::to_value(&email).unwrap();
    let email_struct: Email = serde_json::from_value(json_value).unwrap();

    if !email_struct.email.contains("@") {
        return Err((
            StatusCode::BAD_REQUEST,
            "Failed to process email".to_string(),
        ));
    }
   
    let locked_state = state.lock().await;
    locked_state.add_new_email_list(email.email.clone(), chrono::Utc::now().to_string()).await.unwrap();
    Ok(Json(String::from("Process succeed")))
}

pub async fn get_all_email_list(State(state): State<Arc<Mutex<SubscriberAppState>>>) -> Result<Json<Vec<Subscriber>>, impl IntoResponse> {
    let locked_state = state.lock().await;
    let vec = locked_state.get_all_email_list().await;
   
    if vec.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Email list is empty".to_string(),
        ));
    }
   
    Ok(Json(vec))
}


pub async fn delete_email(State(state): State<Arc<Mutex<SubscriberAppState>>>, Path(id): Path<i32>) -> Result<Json<String>, impl IntoResponse> {
    let locked_state = state.lock().await;
    let result = locked_state.delete_email(id).await;

    
    if result.is_err() {
        return  Err((StatusCode::BAD_REQUEST, result.unwrap_err().to_string()));
    }

    return Ok(Json(String::from("Delete operation completed")));
}

pub fn routes(state: Arc<Mutex<SubscriberAppState>>) -> Router {
    Router::new()
        .route("/subscribe", post(subscribe))
        .route("/subscribers", get(get_all_email_list))
        .route("/subscriber/:id", delete(delete_email))
        .with_state(state)
}