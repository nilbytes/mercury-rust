use super::Result;
use reqwest::Method;
use reqwest::header::{USER_AGENT, AUTHORIZATION};
use crate::error::{Error, ErrorResponse};

#[derive(Clone)]
pub struct Client {
    key: String,
}

impl Client {
    pub fn new<S: AsRef<str>>(api: S) -> Client {
        Client {
            key: api.as_ref().to_string(),
        }
    }

    pub async fn get<A, B>(&self, path: &str, param: Vec<&str>, data: B) -> Result<A>
        where
            A: serde::de::DeserializeOwned + Send + 'static,
            B: serde::Serialize,
    {
        self.request(Method::GET, path, param, data).await
    }

    pub async fn post<A, B>(&self, path: &str, param: Vec<&str>, data: B) -> Result<A>
        where
            A: serde::de::DeserializeOwned + Send + 'static,
            B: serde::Serialize,
    {
        self.request(Method::POST, path, param, data).await
    }

    pub async fn put<A, B>(&self, path: &str, param: Vec<&str>, data: B) -> Result<A>
        where
            A: serde::de::DeserializeOwned + Send + 'static,
            B: serde::Serialize,
    {
        self.request(Method::PUT, path, param, data).await
    }


    pub async fn delete<A, B>(&self, path: &str, param: Vec<&str>, data: B) -> Result<A>
        where
            A: serde::de::DeserializeOwned + Send + 'static,
            B: serde::Serialize,
    {
        self.request(Method::DELETE, path, param, data).await
    }

    pub async fn request<A, B>(
        &self,
        method: Method,
        path: &str,
        param: Vec<&str>,
        data: B,
    ) -> Result<A>
        where
            A: serde::de::DeserializeOwned + Send + 'static,
            B: serde::Serialize,
    {
        let mut param = param
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join("/");


        if param.len() > 0 {
            param = format!("/{}", param);
        }
        let client = reqwest::Client::new();

        let uri = format!("https://backend.mercury.com/api/v1{}{}", path, param);

        let req = client
            .request(method, &uri)
            .json(&data)
            .header(AUTHORIZATION, format!("Bearer {}", self.key.clone()))
            .header(USER_AGENT, "mercury-client/rust");

        let res = req.send().await?;

        return if res.status().is_success() {
            res.json().await.map_err(super::error::Error::from)
        } else {
            let res_body = res.json::<ErrorResponse>().await?;
            Err(Error::MercuryError { errors: res_body.errors })
        }

    }
}