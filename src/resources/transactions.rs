use chrono::{DateTime, Utc};
use crate::resources::{Address, List};
use crate::client::Client;
// use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct Transaction {
    pub amount: f32,
    pub bank_description: Option<String>,
    pub counterparty_id: String,
    pub counterparty_name: String,
    pub counterparty_nickname: Option<String>,
    pub created_at: DateTime<Utc>,
    pub dashboard_link: String,
    pub details: TransactionDetails,
    pub estimated_delivery_date: DateTime<Utc>,
    pub failed_at: Option<DateTime<Utc>>,
    pub id: String,
    pub kind: TransactionDetailKind,
    pub note: Option<String>,
    pub external_memo: Option<String>,
    pub posted_at: Option<DateTime<Utc>>,
    pub reason_for_failure: Option<String>,
    pub status: TransactionDetailStatus,
    pub fee_id: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct TransactionDetails {
    pub address: Option<Address>,
    pub domestic_wire_routing_info: Option<DomesticWireRoutingInfo>,
    pub electronic_routing_info: Option<ElectronicRoutingInfo>,
    pub international_wire_routing_info: Option<InternationalWireRoutingInfo>,

}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all="camelCase")]
pub enum TransactionDetailStatus {
    Pending,
    Sent,
    Cancelled,
    Failed,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all="camelCase")]
pub enum TransactionDetailKind {
    ExternalTransfer,
    InternalTransfer,
    OutgoingPayment,
    DebitCardTransaction,
    IncomingDomesticWire,
    CheckDeposit,
    IncomingInternationalWire,
    Fee,
    Other
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct DomesticWireRoutingInfo {
    pub bank_name: Option<String>,
    pub account_number: String,
    pub routing_number: String,
    pub address: Option<Address>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct ElectronicRoutingInfo {
    pub account_number: String,
    pub routing_number: String,
    pub bank_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct InternationalWireRoutingInfo {
    pub iban: String,
    pub swift_code: String,
    pub correspondent_info: Option<CorrespondentInfo>,
    pub bank_details: Option<BankDetails>,
    pub address: Option<Address>,
    pub phone_number: Option<String>,
    pub country_specific: Option<CountrySpecific>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct CorrespondentInfo {
    pub routing_number: Option<String>,
    pub swift_code: Option<String>,
    pub bank_name: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct BankDetails {
    pub bank_name: String,
    pub city_state: String,
    pub country: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct CountrySpecific {
    pub country_specific_data_canada: Option<CountrySpecificData>,
    pub country_specific_data_australia: Option<CountrySpecificData>,
    pub country_specific_data_india: Option<CountrySpecificData>,
    pub country_specific_data_russia: Option<CountrySpecificData>,
    pub country_specific_data_philippines: Option<CountrySpecificData>,
    pub country_specific_data_south_africa: Option<CountrySpecificData>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all="camelCase")]
pub enum CountrySpecificData {
    Canada { bank_code: String, transit_number: String },
    Australia { bsb_code: String },
    India { ifsc_code: String },
    Russia { inn: String },
    Philippines { routing_number: String },
    SouthAfrica { branch_code: String }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="camelCase")]
pub struct TransactionParam<'a> {
    pub recipient_id: &'a str,
    pub amount: f32,
    pub payment_method: TransactionPaymentMethod,
    pub note: Option<&'a str>,
    pub external_memo: Option<&'a str>,
    pub idempotency_key: &'a str
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all="lowercase")]
pub enum TransactionPaymentMethod {
    ACH
}

impl Transaction {

    pub async fn list(client: &Client, id: &str) -> crate::Result<List<Self>> {
        client.get("/account", vec![id, "transactions"], serde_json::Map::new()).await
    }

    pub async fn retrieve(client: &Client, aid: &str, tid: &str) -> crate::Result<List<Self>> {
        client.get("/account", vec![aid, "transactions", tid], serde_json::Map::new()).await
    }

    pub async fn create<B: serde::Serialize>(client: &Client, aid: &str, param: B) -> crate::Result<Self> {
        client.get("/account", vec![aid, "transactions"], param).await
    }

}