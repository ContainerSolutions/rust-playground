#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate prometheus;
#[macro_use] extern crate lazy_static;

use prometheus::{Counter, TextEncoder, Encoder};
use warp::Filter;

mod handlers;

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

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let index =  warp::path::end()
        .and(warp::get())
        .map(|| {
            MAIN_HTTP_REQUESTS.inc();
            "rq: OK"
        });

    let playground =  warp::path("playground")
        .and(warp::get())
        .map(|| {
        PLAYGROUND_HTTP_REQUESTS.inc();
        "rq: Playground OK"
    });

    let metrics =  warp::path("metrics")
        .and(warp::get())
        .and_then(handlers::metrics);

    let routes = index
        .or(metrics)
        .or(playground);


    warp::serve(routes)
        .run(([127, 0, 0, 1], 8000))
        .await;
}


