#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug)]
#[serde(rename_all = "camelCase")]
pub struct Email {
    pub email: String
}