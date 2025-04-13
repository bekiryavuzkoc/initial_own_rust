use sqlx::prelude::FromRow;

#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Subscriber {
    pub id: i32,
    pub email: String,
    pub subscribed_at: String,
}


#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    pub email: String
}