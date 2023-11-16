mod exchanges;
use exchanges::bitpanda::test_bitpanda;

extern crate colorful;
use colorful::Color;
use colorful::Colorful;

pub fn test_scrap() {
    let test_color = "test scrapper shit.";
    println!("{}", test_color.color(Color::Blue).bold());
    test_bitpanda();
}
