use futures::{Future, Stream};
use hyper_tls::HttpsConnector;
use hyper::Client;
use tokio_core::reactor::Core;
use serde_json;

use std::string::String;
use std::str;
use std;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CurrencyPrice {
    pub USD: f64,
    pub EUR: f64,
}

pub struct Crawler {
}

impl Crawler {
    pub fn new() -> Crawler {
        let c = Crawler {};
        return c;
    }

    fn parse_content (&self, content : String) -> CurrencyPrice {
        let s_slice: &str = &*content;
        let v: CurrencyPrice = serde_json::from_str(s_slice).unwrap();
        return v;
    }


    fn http_get(&self, url : &str) -> String {
        let mut core = Core::new().unwrap();
        let client = Client::configure()
                .connector(HttpsConnector::new(4, &core.handle()).unwrap())
                    .build(&core.handle());

        let uri = url.parse().unwrap();
        let work = client.get(uri).and_then (|res| {res.body().concat2()})
            .map(|chunk| std::str::from_utf8(&chunk).expect("error handling").to_string());
        let r = core.run(work).unwrap();
        return r;
    }


    pub fn get_price (&self, currency : &str) -> CurrencyPrice {
        let url = format!("https://min-api.cryptocompare.com/data/price?fsym={}&tsyms=USD,EUR", currency);

        let content = self.http_get(&url[..]); 
        let values : CurrencyPrice  = self.parse_content(content);
        return values;
    }
}




