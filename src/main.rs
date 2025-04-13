mod controllers;
mod appstate;

use axum::Router;
use sqlx::{sqlite, Executor};
use std::net::SocketAddr;
use tokio::sync::Mutex;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let opt = sqlite::SqliteConnectOptions::new().filename("subscribers.db").create_if_missing(true);
    let connection = sqlite::SqlitePool::connect_with(opt).await.unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&connection)
        .await;
    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }
    println!("migration: {:?}", migration_results);

    let app_state = Arc::new(Mutex::new(appstate::SubscriberAppState::new(connection)));
    
    let app: Router = Router::new().with_state(app_state.clone())
        .nest("/email", controllers::subscriber::routes(app_state.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    
    axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}