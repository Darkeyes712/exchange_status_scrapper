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

    let selector = Selector::parse(".component-inner-container").unwrap();

    let mut bitpanda_components: Vec<BitpandaComponent> = Vec::new();

    for element in document.select(&selector) {
        let name_selector = Selector::parse(".name").unwrap();
        let status_selector = Selector::parse(".component-status").unwrap();

        let mut bitpanda_component = BitpandaComponent::new();

        bitpanda_component.name = element
            .select(&name_selector)
            .next()
            .map(|e| e.text().collect::<String>());
        bitpanda_component.status = element
            .select(&status_selector)
            .next()
            .map(|e| e.text().collect::<String>());

        bitpanda_components.push(bitpanda_component);
    }

    Ok(bitpanda_components)
}
