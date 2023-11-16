mod scrapper;
use scrapper::bitpanda_format;

use tokio;

#[tokio::main]
async fn main() {
    bitpanda_format().await;
}
