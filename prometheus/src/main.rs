#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
// use rocket::routes;
use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder};

#[get("/")]
fn index() -> String {
    // Create a Counter.
    let counter_opts = Opts::new("test_counter", "test counter help");
    let counter = Counter::with_opts(counter_opts).unwrap();

    // Create a Registry and register Counter.
    let r = Registry::new();
    r.register(Box::new(counter.clone())).unwrap();

    // Inc.
    counter.inc();
    counter.inc();

    // Gather the metrics.
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = r.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    // Output to the standard output.
    // println!("{}", String::from_utf8(buffer).unwrap());
    return String::from_utf8(buffer).unwrap()


}

#[tokio::main]
async fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
