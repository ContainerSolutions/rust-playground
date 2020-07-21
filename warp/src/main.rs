//  /v2/<name>/
//
//  can be:
//
//      /v2/<name>/manifests/<reference>
//      /v2/<name>/blobs/<digest>
//  
//  
//  <name> Should handle
//  /v2/redis/manifests
//  /v2/redis/redis/manifests
//  /v2/org/redis/redis/manifests
//  etc


use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let root =  warp::path::end().map(|| "Rust Playground - warp");

    let v2  = warp::path("v2").and(warp::path::end())
        .map(||
            "OCI root"
        );
    
    // GET manifests
    let manifest_one = warp::path!(String / "manifests" / String)
        .map(|name, reference |
            format!("name = {} reference = {}",name ,reference )
            );

    let manifest_two = warp::path!( String / String / "manifests" / String)
        .map(|user, repository, reference| 
            format!("name = {}/{} reference = {}", user, repository, reference)    
        );

    let manifest_three = warp::path!(String / String / String / "manifests" /String)
        .map(|org, user , repository , reference| 
            format!("name = {}/{}/{} reference = {}", org, user, repository,  reference)    
        );
    let manifest_four = warp::path!(String/ String / String / String / "manifests" /String)
        .map(|fourth, org, user, repository, reference| 
            format!("name = {}/{}/{}/{} reference = {}",fourth , org, user, repository,  reference) );
    
    let manifests = warp::path("v2").and(manifest_one.or(manifest_two).or(manifest_three).or(manifest_four));

    // GET blobs      
    let blob_one = warp::path!(String / "blobs" / String)
        .map(|name, digest |
            format!("name = {} digest = {}",name ,digest )
            );

    let blob_two = warp::path!( String / String / "blobs" / String)
        .map(|user, repository, digest| 
            format!("name = {}/{} digest = {}", user, repository, digest)    
        );

    let blob_three = warp::path!(String / String / String / "blobs" /String)
        .map(|org, user , repository , digest| 
            format!("name = {}/{}/{} digest = {}", org, user, repository,  digest)    
        );
    let blob_four = warp::path!(String / String / String / String / "blobs" /String)
        .map(|fourth, org, user, repository, digest| 
            format!("name = {}/{}/{}/{} digest = {}",fourth , org, user, repository,  digest) );
    
    let blobs = warp::path("v2").and(blob_one.or(blob_two).or(blob_three).or(blob_four));

    let routes = warp::get().and(root.or(v2).or(manifests).or(blobs));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

}

