use prometheus::{Encoder, TextEncoder};
use serde::Serialize;
use warp::http::StatusCode;

#[derive(Serialize)]
struct Response {
    code: u16,
    message: String,
}

fn track_status_code(status_code: usize) {
    match status_code {
        500..=599 => crate::RESPONSE_CODES
            .with_label_values(&[&status_code.to_string(), "500"])
            .inc(),
        400..=499 => crate::RESPONSE_CODES
            .with_label_values(&[&status_code.to_string(), "400"])
            .inc(),
        300..=399 => crate::RESPONSE_CODES
            .with_label_values(&[&status_code.to_string(), "300"])
            .inc(),
        200..=299 => crate::RESPONSE_CODES
            .with_label_values(&[&status_code.to_string(), "200"])
            .inc(),
        100..=199 => crate::RESPONSE_CODES
            .with_label_values(&[&status_code.to_string(), "100"])
            .inc(),
        _ => (),
    };
}

pub async fn index() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let code;
    let message;

    info!("index");
    crate::INDEX_REQUESTS_TOTAL.inc();
    crate::REQUESTS_TOTAL.inc();

    message = format!("rq: {}", crate::VERSION);
    code = StatusCode::OK;

    let json = warp::reply::json(&Response {
        code: code.as_u16(),
        message: message.into(),
    });

    track_status_code(code.as_u16().into());

    Ok(Box::new(warp::reply::with_status(json, code)))
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
    let code;
    let message;

    info!("playground");
    crate::PLAYGROUND_REQUESTS_TOTAL.inc();
    crate::REQUESTS_TOTAL.inc();

    message = format!("playground:{}", crate::VERSION);
    code = StatusCode::OK;

    let json = warp::reply::json(&Response {
        code: code.as_u16(),
        message: message.into(),
    });

    track_status_code(code.as_u16().into());

    Ok(Box::new(warp::reply::with_status(json, code)))
}

pub async fn version() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let code;
    let message;

    info!("version");
    crate::REQUESTS_TOTAL.inc();

    message = format!("{}", crate::VERSION);
    code = StatusCode::OK;

    let json = warp::reply::json(&Response {
        code: code.as_u16(),
        message: message.into(),
    });

    track_status_code(code.as_u16().into());

    Ok(Box::new(warp::reply::with_status(json, code)))
}

pub async fn health() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let code;
    let message;

    info!("health");

    crate::REQUESTS_TOTAL.inc();

    message = format!("Ok: {}", crate::VERSION);
    code = StatusCode::OK;

    let json = warp::reply::json(&Response {
        code: code.as_u16(),
        message: message.into(),
    });

    track_status_code(code.as_u16().into());

    Ok(Box::new(warp::reply::with_status(json, code)))
}

pub async fn bad() -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let code;
    let message;

    info!("bad");

    crate::REQUESTS_TOTAL.inc();

    message = format!("Bad: {}", crate::VERSION);
    code = StatusCode::BAD_REQUEST;

    let json = warp::reply::json(&Response {
        code: code.as_u16(),
        message: message.into(),
    });

    track_status_code(code.as_u16().into());

    Ok(Box::new(warp::reply::with_status(json, code)))
}
