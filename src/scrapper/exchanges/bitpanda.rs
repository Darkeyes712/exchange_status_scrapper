use super::config::{Exchange, ExchangeURLs, ServiceStatus};
use async_trait::async_trait;
use reqwest;
use reqwest::Client;
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Bitpanda;

#[async_trait]
impl Exchange for Bitpanda {
    async fn get_data(&self) -> Result<Vec<ServiceStatus>, Box<dyn std::error::Error>> {
        let urls = ExchangeURLs::new();
        let url = urls.bitpanda;

        let reqwest_client = Client::new();
        let response = reqwest_client.get(url).send().await?;
        let html_content = response.text().await?;

        let document = Html::parse_document(&html_content);
        let component_inner_container_selector = ".component-inner-container";

        let mut bitpanda_components: Vec<ServiceStatus> = Vec::new();

        for element in
            document.select(&Selector::parse(component_inner_container_selector).unwrap())
        {
            let name_selector = ".name";
            let status_selector = ".component-status";

            let mut bitpanda_component = ServiceStatus::new();

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
}
