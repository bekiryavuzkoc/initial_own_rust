mod controllers;
mod appstate;

use axum::Router;
use std::net::SocketAddr;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let email_db = Vec::<String>::new();
    let app_state = Arc::new(Mutex::new(appstate::EmailState::new(email_db)));
    
    let app: Router = Router::new().with_state(app_state.clone())
        .nest("/email", controllers::subscriber::routes(app_state.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}