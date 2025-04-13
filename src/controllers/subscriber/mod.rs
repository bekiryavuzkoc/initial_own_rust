pub mod route;
pub mod data;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::appstate::SubscriberAppState;

pub fn routes(state: Arc<Mutex<SubscriberAppState>>) -> Router {
    return route::routes(state)
}