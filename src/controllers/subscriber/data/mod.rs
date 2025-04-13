#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Debug)]
#[serde(rename_all = "camelCase")]
pub struct Subscriber {
    pub email: String
}