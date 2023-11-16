mod scrapper;
use scrapper::bitpanda_format;

#[tokio::main]
async fn main() {
    bitpanda_format().await;
}
