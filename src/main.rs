mod scrapper;
use scrapper::exchanges::bitpanda::Bitpanda;
use scrapper::format_output;

#[tokio::main]
async fn main() {
    let bitpanda = Bitpanda;
    format_output(&bitpanda).await;
}
