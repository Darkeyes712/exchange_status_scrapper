extern crate colorful;
use colorful::Color;
use colorful::Colorful;

pub fn test_bitpanda() {
    let test_color = "Test scrapper shit.";
    println!("{}", test_color.color(Color::Red).bold());
}
