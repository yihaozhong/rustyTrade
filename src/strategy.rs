// strategy.rs: Implements the trading strategy logic.

use crate::stock::Stock;

pub fn decide_action(stock: &Stock) -> String {
    let avg = stock.moving_average();
    let current_price = stock.current_price();

    if current_price > avg {
        "BUY".to_string()
    } else if current_price < avg {
        "SELL".to_string()
    } else {
        "HOLD".to_string()
    }
}
