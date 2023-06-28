use crate::{http::EasyDonateHTTPClient, EasyDonateConfig};

mod endpoints {
    const SHOP_INFO: &str = "/shop";
    const PRODUCTS: &str = "/shop/products";
    const PRODUCT_INFO: &str = "/shop/product/{id}";
}

pub struct EasyDonate<H: EasyDonateHTTPClient> {
    config: EasyDonateConfig,
    http: H,
}

impl<H: EasyDonateHTTPClient> EasyDonate<H> {
    pub fn new(config: EasyDonateConfig) -> Self {
        let http = H::create(config.api_key.clone());
        Self { config, http }
    }

    pub async fn shop_info(&self) -> Result<ShopInfo, EasyDonateError> {}
}
