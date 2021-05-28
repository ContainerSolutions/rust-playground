// #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate prometheus;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use prometheus::Counter;
use warp::Filter;

mod handlers;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

lazy_static! {
    static ref PLAYGROUND_HTTP_REQUESTS: Counter = register_counter!(opts!(
        "playground_http_requests_total",
        "Total number of HTTP requests made to /playground.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    static ref MAIN_HTTP_REQUESTS: Counter = register_counter!(opts!(
        "main_http_requests_total",
        "Total number of HTTP requests made to /.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let index = warp::path::end().and(warp::get()).and_then(handlers::index);

    let version = warp::path("version")
        .and(warp::get())
        .and_then(handlers::version);

    let playground = warp::path("playground")
        .and(warp::get())
        .and_then(handlers::playground);

    let metrics = warp::path("metrics")
        .and(warp::get())
        .and_then(handlers::metrics);

    let routes = index.or(metrics).or(playground).or(version);

    info!("rq starting: {}", VERSION);
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
