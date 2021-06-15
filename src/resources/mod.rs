pub mod accounts;
pub mod cards;
pub mod transactions;
pub mod statements;
pub mod recipient;


#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Address {
    pub address1: String,
    pub address2: Option<String>,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: Option<String>,
    pub region: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct List<T> {
    pub total: Option<i64>,
    #[serde(flatten)]
    pub data: ListData<T>
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all="lowercase")]
pub enum ListData<T> {
    Accounts(Vec<T>),
    Cards(Vec<T>),
    Transactions(Vec<T>),
    Recipients(Vec<T>),
    Statements(Vec<T>)
}