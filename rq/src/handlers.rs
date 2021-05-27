
use prometheus::{TextEncoder, Encoder};

pub async fn metrics() -> Result<Box<dyn warp::Reply>, warp::Rejection> {

    let encoder = TextEncoder::new();
        
    let metric_families = prometheus::gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    
    let res = String::from_utf8(buffer).unwrap();


    return Ok(Box::new(res));
}
