// util.rs

use serde_json::{Value};
use std::collections::VecDeque;

pub fn extract_prices(data: &Value) -> Option<VecDeque<f64>> {
    let time_series = data.get("Time Series (5min)")?;

    time_series.as_object()
        .map(|entries| {
            entries.iter()
                .filter_map(|(_timestamp, values)| {
                    if let Some(close_price) = values.get("4. close") {
                        close_price.as_str()?.parse::<f64>().ok()
                    } else {
                        None
                    }
                })
                .collect::<VecDeque<_>>()
        })
}
