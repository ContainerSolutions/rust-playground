#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate prometheus;
#[macro_use] extern crate lazy_static;

use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};

lazy_static! {
    static ref PLAYGROUND_HTTP_REQUESTS: Counter = register_counter!(opts!(
        "playground_http_requests_total",
        "Total number of HTTP requests made to /playground.",
        labels! {"handler" => "all",}
    )).unwrap();
    static ref MAIN_HTTP_REQUESTS: Counter = register_counter!(opts!(
        "main_http_requests_total",
        "Total number of HTTP requests made to /.",
        labels! {"handler" => "all",}
    )).unwrap();
}

#[get("/")]
fn index() -> String {
    // Inc.
    MAIN_HTTP_REQUESTS.inc();

    // Gather the metrics.
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    return String::from_utf8(buffer).unwrap()
}

#[get("/playground")] 
fn playground() -> String {

    let encoder = TextEncoder::new();

    PLAYGROUND_HTTP_REQUESTS.inc();
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    return String::from_utf8(buffer).unwrap()
}

#[tokio::main]
async fn main() {
    rocket::ignite().mount("/", routes![index, playground]).launch();
}
