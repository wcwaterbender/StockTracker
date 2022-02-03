use yahoo_finance_api as yahoo;
use std::time::{Duration, UNIX_EPOCH};
use chrono::prelude::*;
use std::error::Error;
pub struct StockTracker {
    provider: yahoo::YahooConnector,
}

impl StockTracker {
    
    pub fn new() -> Self {
        Self {
            provider: yahoo_finance_api::YahooConnector::new(),
        }
    }

    // grabs closing quote for stock at 1 min interval
    pub fn get_stock_info(&self, stock: &String) -> Result<(), Box<dyn Error>> {
        // get the latest quotes in 1 minute intervals
        let response =  
            match tokio_test::block_on(self.provider.get_latest_quotes(&stock, "1m")) {
                Ok(resp) => resp,
                Err(_) => return Err("please enter a valid stock ticker symbol")?
            };
        // extract just the latest valid quote summery
        // including timestamp,open,close,high,low,volume
        let quote = response.last_quote().unwrap();
        let time: DateTime<Utc> =
            DateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
        println!("At {} quote price of {} was {}", time.to_rfc3339(), &stock, quote.close);
        Ok(())
    }

    
}