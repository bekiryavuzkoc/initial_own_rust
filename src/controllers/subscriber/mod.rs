pub mod route;
pub mod data;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::appstate::EmailState;

pub fn routes(state: Arc<Mutex<EmailState>>) -> Router {
    return route::routes(state)
}