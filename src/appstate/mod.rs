use app_state::{AppState, MutAppState, AppStateTrait, InitAppState, InitMutAppState};
use sqlx::SqlitePool;

use crate::controllers::subscriber::data::Subscriber;

#[derive(InitAppState, InitMutAppState)]
pub struct SubscriberAppState {
    pub db: SqlitePool,
}

impl SubscriberAppState {
    pub fn new(db: SqlitePool) -> Self {
        SubscriberAppState { db: db }
    }

    pub async fn get_all_email_list(&self) -> Vec<Subscriber> {
        return sqlx::query_as::<_, Subscriber>(
            "SELECT * FROM subscribers"
        ).fetch_all(&self.db).await.unwrap();
    }

    pub async fn add_new_email_list(&self, email: String, time: String) -> Result<(), sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO subscribers (email, subscribed_at) VALUES (?, ?)"
        )
        .bind(email)
        .bind(time)
        .execute(&self.db)
        .await;
    
        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to insert new subscriber: {}", e);
                Err(e)
            }
        }
    }

    pub async fn delete_email(&self, id: i32) -> Result<(), sqlx::Error> {
        let result = sqlx::query("DELETE FROM subscribers WHERE id = ?")
        .bind(id)
        .execute(&self.db).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to delete the subscriber: {}", e);
                Err(e)
            }
        }
    }
}