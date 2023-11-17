pub struct ExchangeURLs {
    pub bitpanda: &'static str,
}

impl ExchangeURLs {
    pub fn new() -> Self {
        ExchangeURLs {
            bitpanda: "https://status.bitpanda.com/",
        }
    }
}

pub trait ExchangeConfig {
    fn get_component() -> &'static str;
    fn get_name_selector() -> &'static str;
    fn get_status_selector() -> &'static str;
}

pub struct BitpandaConfig;

impl ExchangeConfig for BitpandaConfig {
    fn get_component() -> &'static str {
        ".component-inner-container"
    }

    fn get_name_selector() -> &'static str {
        ".name"
    }

    fn get_status_selector() -> &'static str {
        ".component-status"
    }
}
