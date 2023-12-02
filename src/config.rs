// config.rs: Manages configuration settings and command-line arguments.

use crate::stock::Stock;

pub struct Config {
    pub symbols: Vec<String>,
    pub window_size: usize,
    pub update_interval: u64, // in seconds
}

pub fn parse_args() -> Config {
    // This is a placeholder implementation. You should use a command-line argument parser here.
    Config {
        symbols: vec!["AAPL".to_string(), "MSFT".to_string()], // example stock symbols
        window_size: 5,
        update_interval: 60,
    }
}

pub fn initialize_stocks(config: &Config) -> Vec<Stock> {
    config
        .symbols
        .iter()
        .map(|symbol| Stock::new(symbol.clone(), config.window_size))
        .collect()
}
