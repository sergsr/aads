// TODO: says premature end of input
// use chrono::{DateTime, NaiveDateTime};
// use chrono::offset::Utc;
use futures::{future, Future, Stream};
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use serde_json;

type HttpsClient = Client<HttpsConnector<HttpConnector>, Body>;

// TODO: public static strongly type URI...

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    adjusted_previous_close: String,
    ask_price: String,
    ask_size: u32,
    bid_price: String,
    bid_size: u32,
    has_traded: bool,
    // TODO: can this be a URL? needs to implement Serde's deserialize
    instrument: String,
    last_extended_hours_trade_price: String,
    last_trade_price: String,
    last_trade_price_source: String,
    previous_close: String,
    previous_close_date: String,
    symbol: String,
    trading_halted: bool,
    updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quotes {
    results: Vec<Quote>,
}

pub fn get_quotes(
    client: &HttpsClient,
    symbols: &Vec<String>,
) -> impl Future<Item = Quotes, Error = ()> {
    let query = &symbols.join(",").to_uppercase();
    let url = format!("https://api.robinhood.com/quotes/?symbols={}", query);
    client
        .get(url.parse().unwrap())
        .map_err(|e| println!("http client error: {}", e))
        .and_then(|res| match res.status().is_success() {
            true => future::ok(res),
            // TODO: return the status code as an actual error
            false => future::err(println!("bad status code: {}", res.status())),
        })
        .and_then(|res| {
            res.into_body()
                .concat2()
                .map_err(|e| println!("error collecting body: {}", e))
                .and_then(|body| serde_json::from_slice(&body).map_err(|e| println!("{}", e)))
        })
}
