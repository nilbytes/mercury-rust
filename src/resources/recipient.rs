use crate::client::Client;
use crate::resources::transactions::{
    DomesticWireRoutingInfo, InternationalWireRoutingInfo,
};
use crate::resources::{Address, List};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Recipient {
    pub id: String,
    pub name: String,
    pub email: Vec<String>,
    pub date_last_paid: Option<DateTime<Utc>>,
    pub electronic_routing_number: ElectronicRoutingInfo,
    pub domestic_wire_routing_info: Option<DomesticWireRoutingInfo>,
    pub international_wire_routing_info: Option<InternationalWireRoutingInfo>,
    pub address: Option<Address>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ElectronicRoutingInfo {
    pub account_number: String,
    pub routing_number: String,
    pub bank_name: Option<String>,
    pub electronic_account_type: ElectronicAccountType,
    pub address: Option<Address>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ElectronicAccountType {
    BusinessChecking,
    BusinessSavings,
    PersonalChecking,
    PersonalSavings,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DefaultPaymentMethod {
    ACH,
    Check,
    DomesticWire,
    InternationalWire,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RecipientStatus {
    Active,
    Deleted,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RecipientPaymentMethod {
    Check,
    Electronic,
    DomesticWire,
    InternationalWire,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecipientParam<'a> {
    pub name: &'a str,
    pub address: Address,
    pub emails: Vec<&'a str>,
    pub payment_method: RecipientPaymentMethod,
    pub electronic_routing_info: ElectronicRoutingInfo,
    pub domestic_wire_routing_info: DomesticWireRoutingInfo,
    pub international_wire_routing_info: InternationalWireRoutingInfo,
}

impl Recipient {
    pub async fn list(client: &Client) -> crate::Result<List<Self>> {
        client
            .get("/recipients", vec![], serde_json::Map::new())
            .await
    }

    pub async fn retrieve(client: &Client, id: &str) -> crate::Result<Self> {
        client
            .get("/recipients", vec![id], serde_json::Map::new())
            .await
    }

    pub async fn create<B: serde::Serialize>(client: &Client, param: B) -> crate::Result<Self> {
        client.get("/recipients", vec![], param).await
    }

    pub async fn update<B: serde::Serialize>(
        client: &Client,
        id: &str,
        param: B,
    ) -> crate::Result<Self> {
        client.get("/recipients", vec![id], param).await
    }
}
