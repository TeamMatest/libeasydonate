use std::sync::Arc;

use reqwest::Client;

use crate::EasyDonateHTTPClient;

pub struct EasyDonateReqwest {
    client: reqwest::Client,
    api_key: Arc<str>,
}

impl EasyDonateHTTPClient for EasyDonateReqwest {
    fn create(api_key: std::sync::Arc<str>) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
}
