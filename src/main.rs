// main.rs: The entry point of the application.

mod stock;
mod api;
mod strategy;
mod config;
mod util;

#[tokio::main]
async fn main() {
    let config = config::parse_args();
    let mut stocks = config::initialize_stocks(&config);

    loop {
        for stock in stocks.iter_mut() {
            if let Err(e) = stock.update_price().await {
                eprintln!("Error updating price for {}: {}", stock.symbol, e);
                continue;
            }

            let action = strategy::decide_action(stock);
            util::log_action(&stock.symbol, stock.current_price(), &action);
        }
    }
}
