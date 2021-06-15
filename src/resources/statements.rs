use crate::resources::{Address, List};
use chrono::{DateTime, Utc};
use crate::client::Client;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Statement {
    pub account_number: String,
    pub company_legal_address: Address,
    pub company_legal_name: String,
    pub ein: String,
    pub end_date: DateTime<Utc>,
    pub ending_balance: f32,
    pub routing_number: Option<String>,
    pub start_date: DateTime<Utc>
}

impl Statement {

    pub async fn list(client: &Client, id: &str) -> crate::Result<List<Self>> {
        client.get("/account", vec![id, "statements"], serde_json::Map::new()).await
    }

}