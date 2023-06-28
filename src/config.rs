use std::sync::Arc;

const DEFAULT_BASE_URL: &str = "https://easydonate.ru/api";

#[derive(Clone, Debug)]
pub struct EasyDonateConfig {
    pub base_url: String,
    pub(crate) api_key: Arc<str>,
    #[allow(dead_code)]
    what: (),
}

impl EasyDonateConfig {
    pub fn new(api_key: &str) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.into(),
            api_key: api_key.into(),
            what: (),
        }
    }
}
