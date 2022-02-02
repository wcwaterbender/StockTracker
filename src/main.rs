use yahoo_finance_api as yahoo;
use std::time::{Duration, UNIX_EPOCH};
use chrono::prelude::*;
use std::error::Error;
use std::env;
extern crate tokio_test;


fn main()-> Result<(), Box<dyn Error>>{
    let provider = yahoo::YahooConnector::new();
    let args: Vec<String> = env::args().collect();

    let stock= &args[1];
    // get the latest quotes in 1 minute intervals
    let response = tokio_test::block_on(provider.get_latest_quotes(&stock, "1m")).unwrap();
    // extract just the latest valid quote summery
    // including timestamp,open,close,high,low,volume
    let quote = response.last_quote().unwrap();
    let time: DateTime<Utc> =
        DateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
    println!("At {} quote price of {} was {}", time.to_rfc3339(), &stock, quote.close);
    Ok(())
}
