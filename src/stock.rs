// stock.rs: Defines the Stock struct and related methods.

use std::collections::VecDeque;
use crate::api;
use crate::util;
use std::error::Error;
pub struct Stock {
    pub symbol: String,
    prices: VecDeque<f64>,
    window_size: usize,
}

impl Stock {
    pub fn new(symbol: String, window_size: usize) -> Self {
        Stock {
            symbol,
            prices: VecDeque::new(),
            window_size,
        }
    }
    pub fn add_price(&mut self, price: f64) {
        self.prices.push_back(price);
        if self.prices.len() > self.window_size {
            self.prices.pop_front();
        }
    }
    pub async fn update_price(&mut self) -> Result<(), Box<dyn Error>> {
        let data = api::fetch_stock_data(&self.symbol).await?;
        if let Some(new_prices) = util::extract_prices(&data) {
            for price in new_prices {
                self.prices.push_back(price);
                if self.prices.len() > self.window_size {
                    self.prices.pop_front();
                }
            }
        }
        Ok(())
    }


    pub fn current_price(&self) -> f64 {
        *self.prices.back().unwrap_or(&0.0)
    }

    // Method to calculate the standard deviation of stock prices
    pub fn price_standard_deviation(&self) -> f64 {
        let mean = self.prices.iter().sum::<f64>() / self.prices.len() as f64;
        let variance = self.prices
            .iter()
            .map(|price| {
                let diff = mean - price;
                diff * diff
            })
            .sum::<f64>() / self.prices.len() as f64;

        variance.sqrt()
    }
    // Additional methods can be added here, like historical price data analysis
}
