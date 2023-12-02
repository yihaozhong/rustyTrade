// main.rs

mod api;
mod stock;
mod util;

use crate::stock::Stock;
use tokio;

#[tokio::main]
async fn main() {
    let symbol = "MSFT"; // Symbol for Microsoft
    let window_size = 10; // Example window size for standard deviation calculation

    // Fetch recent stock data
    match api::fetch_stock_data(symbol).await {
        Ok(data) => {
            // Extract stock prices from data and perform analysis
            if let Some(prices) = util::extract_prices(&data) {
                let mut stock = Stock::new(symbol.to_string(), window_size);
                for price in prices {
                    stock.add_price(price);
                }

                // Calculate and print the standard deviation
                let std_dev = stock.price_standard_deviation();
                println!("Standard Deviation of {}'s stock prices: {}", symbol, std_dev);

                // Additional analysis can be performed here
            } else {
                eprintln!("Failed to extract prices from the fetched data.");
            }
        }
        Err(e) => eprintln!("Error fetching stock data: {}", e),
    }
}
