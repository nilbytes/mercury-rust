use chrono::{DateTime, Utc};
use crate::resources::List;
use crate::client::Client;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Card {
    pub created_at: DateTime<Utc>,
    pub last_four_digits: String,
    pub name_on_card: String,
    pub network: CardNetwork,
    pub status: CardStatus
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all="lowercase")]
pub enum CardNetwork {
    Visa,
    Mastercard
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all="lowercase")]
pub enum CardStatus {
    Active,
    Frozen,
    Cancelled,
    Inactive,
    Locked
}

impl Card {

    pub async fn list(client: &Client, id: &str) -> crate::Result<List<Self>> {
        client.get("/account", vec![id, "cards"], serde_json::Map::new()).await
    }

}