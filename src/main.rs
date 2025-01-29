use crate::api::{CoinData, API};
use dotenv::dotenv;
use std::io::{Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, thread};
use serde::{Deserialize, Serialize};

pub mod api;
pub mod lcd;

#[derive(Serialize, Deserialize)]
struct CoinData {
    pub id: String,
    pub rank: String,
    pub symbol: String,
    pub priceUsd: String,
}

fn main() {
    dotenv().ok();

    let api_url = env::var("API_URL").unwrap_or_else(|_| {
        eprintln!("API_URL must be set");
        std::process::exit(1);
    });

    // initialize everything
    let mut lcd = lcd::Lcd::new();
    lcd.initialize();

    let coin_data = Arc::new(Mutex::new(Vec::<CoinData>::new()));
    let is_fetching = Arc::new(Mutex::new(false)); // flag to check if we are fetching data

    // clone info for api thread
    let coin_clone = Arc::clone(&coin_data);
    let is_fetching_clone = Arc::clone(&is_fetching);

    const data_endpoint: String = "https://api.coincap.io/v2/assets".to_string();

    // API thread
    thread::spawn(move || {
        loop {
            println!("getting new coin data");

            {
                // set fetching to true
                let mut fetching_lock = is_fetching_clone.lock().unwrap();
                *fetching_lock = true;
            }

            match API::make_get::<Vec<CoinData>>(data_endpoint) {
                Ok(new_info) => {
                    let mut coin_lock = coin_clone.lock().unwrap();
                    *coin_lock = new_info;
                }
                Err(e) => {
                    eprintln!("error fetching new info: {:?}", e);
                }
            }

            thread::sleep(Duration::from_millis(500)); // wait a bit before setting fetching to false

            {
                // set fetching to false
                let mut fetching_lock = is_fetching_clone.lock().unwrap();
                *fetching_lock = false;
            }

            println!("done fetching new coin data");
            thread::sleep(Duration::from_secs(600)); // update every 10 minutes
        }
    });

    // clone info for lcd thread
    let coin_clone = Arc::clone(&coin_data);
    let is_fetching_clone = Arc::clone(&is_fetching);

    // LCD thread
    thread::spawn(move || {
        let mut switcher = 0;
        loop {
            {
                let fetching_lock = is_fetching_clone.lock().unwrap();
                if *fetching_lock {
                    lcd.clear_display();
                    lcd.move_to_line(1);
                    lcd.display_message("updating...");
                } else {
                    let coin_lock = coin_clone.lock().unwrap();

                    let current_coin = coin_lock
                        .get(switcher)
                        .map_or("".to_string(), |coin| coin.symbol.clone());

                    let current_price = coin_lock
                        .get(switcher)
                        .map_or("".to_string(), |coin| coin.priceUsd.clone())
                        .parse::<f64>()
                        .unwrap_or(0.0);

                    lcd.clear_display();

                    lcd.move_to_line(1);
                    lcd.display_message(&format!("{}", current_coin));

                    lcd.move_to_line(2);
                    lcd.display_message(&format!("${:.2}", current_price));

                    if switcher >= coin_lock.len() - 1 {
                        switcher = 0;
                    } else {
                        switcher += 1;
                    }
                }
            }
            thread::sleep(Duration::from_secs(2));
        }
    });

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
