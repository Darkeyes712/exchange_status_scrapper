pub mod exchanges;
use exchanges::config::{Exchange, ServiceStatus};

extern crate colorful;
use colorful::Color;
use colorful::Colorful;

pub async fn format_output(exchange: &dyn Exchange) {
    let service_statuses = exchange.get_data().await.unwrap();

    for component in &service_statuses {
        if let (Some(name), Some(status)) = (&component.name, &component.status) {
            let formatted_component = ServiceStatus {
                name: Some(name.clone()),
                status: Some(status.clone()),
            };

            let name_string = formatted_component.name.unwrap().trim().to_string();
            let status_string = formatted_component.status.unwrap().trim().to_string();

            match status_string.as_str() {
                "Operational" => {
                    println!(
                        "{} || {}",
                        name_string,
                        status_string.color(Color::Green).bold(),
                    );
                }
                _ => {
                    println!(
                        "{} || {}",
                        name_string,
                        status_string.color(Color::Red).bold(),
                    );
                }
            }
        }
    }
}
