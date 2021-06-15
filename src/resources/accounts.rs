use chrono::{DateTime, Utc};
use crate::client::Client;
use crate::resources::List;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Account {
    pub account_number: String,
    pub available_balance: f32,
    pub created_at: DateTime<Utc>,
    pub current_balance: f32,
    pub id: String,
    pub kind: AccountKind,
    pub name: String,
    pub routing_number: String,
    pub status: AccountStatus,
    pub r#type: AccountType,
    pub can_receive_transactions: Option<bool>,
    pub nickname: Option<String>,
    pub legal_business_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all="lowercase")]
pub enum AccountKind {
    Checking,
    Savings
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all="lowercase")]
pub enum AccountStatus {
    Active,
    Deleted,
    Pending
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all="lowercase")]
pub enum AccountType {
    Mercury,
    External,
    Recipient
}

impl Account {

    pub async fn list(client: &Client) -> crate::Result<List<Self>> {
        client.get("/accounts", vec![], serde_json::Map::new()).await
    }

    pub async fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client.get("/account", vec![id], serde_json::Map::new()).await
    }

}