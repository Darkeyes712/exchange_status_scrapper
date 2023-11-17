use async_trait::async_trait;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct ExchangeURLs {
    pub bitpanda: &'static str,
}

impl ExchangeURLs {
    pub fn new() -> Self {
        ExchangeURLs {
            bitpanda: "https://status.bitpanda.com/",
        }
    }
}

#[derive(Debug, Default)]
pub struct ServiceStatus {
    pub name: Option<String>,
    pub status: Option<String>,
}

impl ServiceStatus {
    pub fn new() -> Self {
        ServiceStatus {
            name: None,
            status: None,
        }
    }
}

#[async_trait]
pub trait Exchange {
    async fn get_data(&self) -> Result<Vec<ServiceStatus>, Box<dyn std::error::Error>>;
}
