use yahoo_finance_api as yahoo;
use std::time::{Duration, UNIX_EPOCH};
use chrono::prelude::*;
use std::error::Error;
use std::env;
use std::io::prelude::*;
extern crate tokio_test;


fn get_stock_info() -> Result<(), Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        Err("please enter a stock ticker symbol")?
    }
    let stock= &args[1];
    // get the latest quotes in 1 minute intervals
    let response =  
        match tokio_test::block_on(provider.get_latest_quotes(&stock, "1m")) {
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

fn main(){
    let result = get_stock_info();
    if let Err(err) = result {
        let _ = writeln!(std::io::stderr(), "Error: {}", err);
    }
}
