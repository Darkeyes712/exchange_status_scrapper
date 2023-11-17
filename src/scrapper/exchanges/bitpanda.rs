use super::config::{BitpandaConfig, ExchangeConfig};
use reqwest;
use reqwest::Client;
use scraper::{Html, Selector};
use std::fmt::Debug;

#[derive(Debug)]
pub struct BitpandaComponent {
    pub name: Option<String>,
    pub status: Option<String>,
}

impl BitpandaComponent {
    fn new() -> Self {
        BitpandaComponent {
            name: None,
            status: None,
        }
    }
}

pub async fn get_bitpanda_data() -> Result<Vec<BitpandaComponent>, Box<dyn std::error::Error>> {
    let url = "https://status.bitpanda.com/";

    let reqwest_client = Client::new();
    let response = reqwest_client.get(url).send().await?;
    let html_content = response.text().await?;

    let document = Html::parse_document(&html_content);

    let component_inner_container_selector = BitpandaConfig::get_component();

    let mut bitpanda_components: Vec<BitpandaComponent> = Vec::new();

    for element in document.select(&Selector::parse(component_inner_container_selector).unwrap()) {
        let name_selector = BitpandaConfig::get_name_selector();
        let status_selector = BitpandaConfig::get_status_selector();

        let mut bitpanda_component = BitpandaComponent::new();

        bitpanda_component.name = element
            .select(&Selector::parse(name_selector).unwrap())
            .next()
            .map(|e| e.text().collect::<String>());
        bitpanda_component.status = element
            .select(&Selector::parse(status_selector).unwrap())
            .next()
            .map(|e| e.text().collect::<String>());

        bitpanda_components.push(bitpanda_component);
    }

    Ok(bitpanda_components)
}
