pub mod route;
pub mod data;
use axum::Router;

pub fn routes() -> Router {
    return route::routes()
}