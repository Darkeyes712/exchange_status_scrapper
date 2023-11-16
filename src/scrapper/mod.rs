mod exchanges;

use exchanges::bitpanda::test_bitpanda;

pub fn test_scrap() {
    println!("test scrapper shit.");
    test_bitpanda();
}
