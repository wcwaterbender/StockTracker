
use std::env;
use std::io::prelude::*;
use std::thread;
use stock_tracker::StockTracker;
extern crate tokio_test;

fn main(){
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        let _ = writeln!(std::io::stderr(), "Error: please enter a stock ticker symbol");
        return;
    }

    let stock= &args[1];
    let interval = std::time::Duration::from_millis(1000);
    let tracker = StockTracker::new();

    loop {
        let result = tracker.get_stock_info(&stock);
        if let Err(err) = result {
            let _ = writeln!(std::io::stderr(), "Error: {}", err);
            return;
        }
        thread::sleep(interval);
    }
}
