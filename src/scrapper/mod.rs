mod exchanges;
use exchanges::bitpanda::get_bitpanda_data;
use exchanges::bitpanda::BitpandaComponent;

extern crate colorful;
use colorful::Color;
use colorful::Colorful;

pub async fn bitpanda_format() {
    let bitpanda_components = get_bitpanda_data().await.unwrap();

    for component in &bitpanda_components {
        if let (Some(name), Some(status)) = (&component.name, &component.status) {
            let bitpanda_component = BitpandaComponent {
                name: Some(name.clone()),
                status: Some(status.clone()),
            };

            let name_string = bitpanda_component.name.unwrap().trim().to_string();
            let status_string = bitpanda_component.status.unwrap().trim().to_string();

            println!(
                "{} || {}",
                name_string.color(Color::Green).bold(),
                status_string.color(Color::Red).bold(),
            );
        } else {
            ()
        }
    }
}
