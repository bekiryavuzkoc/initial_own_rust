mod controllers;

use axum::Router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/email", controllers::email::routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    

    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}