use prometheus::{Encoder, TextEncoder};

pub async fn index() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    info!("index");
    crate::MAIN_HTTP_REQUESTS.inc();
    let res = format!("rq: {}", crate::VERSION);
    Ok(Box::new(warp::reply::json(&res)))
}

pub async fn metrics() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    info!("metrics");
    let encoder = TextEncoder::new();

    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let res = String::from_utf8(buffer).unwrap();
    info!("metrics: {}", res);

    return Ok(Box::new(res));
}

pub async fn playground() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    info!("playground");
    crate::PLAYGROUND_HTTP_REQUESTS.inc();
    let res = format!("playground:{}", crate::VERSION);
    Ok(Box::new(warp::reply::json(&res)))
}

pub async fn version() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    info!("version");
    let res = format!("{}", crate::VERSION);
    Ok(Box::new(warp::reply::json(&res)))
}
