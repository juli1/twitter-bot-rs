extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate twitter_api;
extern crate oauth_client as oauth;

use std::time;
use std::env;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

mod config;
mod crawler;
mod twitter;

const CONF_FILENAME: &'static str = ".crypto-bot.conf";

fn get_home_dir() -> PathBuf {
    match env::home_dir() {
        Some(p) => p,
        None => {
            panic!("Impossible to get your home dir!");
        }
    }
}

pub fn build_message (prices : &mut HashMap<&str, crawler::CurrencyPrice>) -> String {
    let mut s = String::from("");
    for (cur, value) in prices.iter() {
        let t = format!("#{}: ${} ${}€\n",cur, value.USD, value.EUR);
        s = s + &t[..];
    }

    return s;
}

fn main() {
    let mut conf_file_path: PathBuf = get_home_dir();
    conf_file_path.push(Path::new(CONF_FILENAME));


    let config = match config::Config::read(&conf_file_path) {
        Some(v) => v,
        None => panic!("Cannot find config file"),
    };

    let crawler = crawler::Crawler::new();
    let twitter = twitter::Twitter::new(config.consumer_key, config.consumer_secret,
                                        config.access_key, config.access_secret);

    //let mut prices : HashMap<&str,crawler::CurrencyPrice>= HashMap::new();
    let mut prices = HashMap::new();

    loop {
        for currency in &config.currencies_to_follow {
            // Get the price of the currency
            let c = &currency[..];
            let price = crawler.get_price(c);
            prices.insert(c, price);
        }
        
        let msg = build_message(&mut prices);
        twitter.tweet(msg);

        std::thread::sleep(time::Duration::from_millis(1000 * config.interval_sec));
    }
}
