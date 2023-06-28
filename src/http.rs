#[cfg(feature = "http-reqwest")]
pub mod reqwest;

use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use serde::Deserialize;
use serde_json::Value;

#[async_trait]
pub trait EasyDonateHTTPClient {
    fn create(api_key: Arc<str>) -> Self;

    async fn request(&self, req: EasyDonateRequest) -> EasyDonateResponse;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EasyDonateRequest {
    pub url: String,
    pub query: HashMap<String, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub enum EasyDonateResponse {
    Success {
        success: bool,
        response: Value,
    },
    Failure {
        success: bool,
        response: Value,
        error_code: i32,
    },
}
