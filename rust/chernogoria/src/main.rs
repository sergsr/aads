extern crate chernogoria;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio;

use chernogoria::robinhood;
use futures::Future;

fn main() {
    let args = chernogoria::args::get_args().unwrap_or_else(|e| e.exit());
    let mut https_connector = hyper_tls::HttpsConnector::new(1).unwrap();
    https_connector.force_https(true);
    let https_client = hyper::Client::builder().build(https_connector);
    let task = robinhood::get_quotes(&https_client, &args.arg_symbol).and_then(|qs| {
        println!("quotes: {:?}", qs);
        Ok(())
    });
    tokio::run(task);
}
