//! Adapted from the Fastly Compute@Edge static content starter kit
//! See https://github.com/fastly/compute-starter-kit-rust-static-content

mod config;

use crate::config::{POP_ORIGIN, DEFAULT_POP, US_ORIGIN, REGION_REGEX, Origin};

use fastly::http::{header, Method};
use fastly::{Error, Request, Response};

cfg_if::cfg_if! {
    if #[cfg(feature = "auth")] {
        mod awsv4;
        use chrono::Utc;
        use crate::awsv4::hash;
        use fastly::handle::dictionary::DictionaryHandle;
    }
}

/// The entry point for the application.
///
/// This function is triggered when the service receives a client request. 
/// It is used to route requests to a bucket in a specific region based on 
/// the edge server ('pop') on which it is running.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    // Where is the application running?
    let pop = get_pop(&req);

    // Choose the origin based on the edge server (pop)
    // Default to US if there is no match on the pop
    let origin = POP_ORIGIN.get(pop.as_str()).unwrap_or(&US_ORIGIN);

    // Remove the query string to improve cache hit ratio
    req.remove_query();

    // Set the `Host` header to the bucket name + host rather than our C@E endpoint.
    let host = format!("{}.{}", origin.bucket_name, origin.bucket_host);
    req.set_header(header::HOST, &host);

    // Copy the modified client request to form the backend request.
    let mut bereq = req.clone_without_body();

    // Set the AWS V4 authentication headers
    set_authentication_headers(&mut bereq, &origin);

    // Send the request to the backend and assign its response to `beresp`.
    let mut beresp = bereq.send(origin.backend_name)?;

    // Set a response header indicating the origin that we used
    beresp.set_header("X-B2-Host", &host);

    // return the response to the client.
    return Ok(beresp);
}

/// Return the three letter identifier of the edge server ('POP') on which
/// the application is running.
fn get_pop(req: &Request) -> String {
    let pop_param = req.get_query_parameter("pop");
    return
        if !pop_param.is_none() {
            // There is a pop query parameter - pretend we are running on
            // that edge server
            pop_param.unwrap().to_string()
        } else if std::env::var("FASTLY_HOSTNAME").unwrap() == "localhost" {
            // Running in the local test server, use the compile-time default
            DEFAULT_POP.to_string()
        } else {
            // The FASTLY_POP environment variable holds a three letter code 
            // representing the edge server on which the application is running. 
            std::env::var("FASTLY_POP").unwrap()
        }; 
}

/// Sets authentication headers for a given request.
#[cfg(feature = "auth")]
fn set_authentication_headers(req: &mut Request, origin: &Origin) {
    // Ensure that request is a GET to prevent signing write operations
    if req.get_method() != Method::GET {
        return;
    }

    let auth = match DictionaryHandle::open("bucket_auth") {
        Ok(h) if h.is_valid() => h,
        _ => return,
    };

    let id = match auth.get(format!("{}{}", origin.backend_name, "_access_key_id").as_str(), 8000) {
        Ok(Some(id)) => id,
        _ => return,
    };
    let key = match auth.get(format!("{}{}", origin.backend_name, "_secret_access_key").as_str(), 8000) {
        Ok(Some(key)) => key,
        _ => return,
    };

    // Extract region from the endpoint
    let bucket_region = REGION_REGEX.find(origin.bucket_host).unwrap().as_str();

    let client = awsv4::SignatureClient {
        access_key_id: id,
        secret_access_token: key,
        bucket_name: origin.bucket_name.to_string(),
        bucket_host: origin.bucket_host.to_string(),
        bucket_region: bucket_region.to_string(),
        query_string: req.get_query_str().unwrap_or("").to_string()
    };

    let now = Utc::now();
    let sig = client.aws_v4_auth(req.get_method().as_str(), req.get_path(), now);
    req.set_header(header::AUTHORIZATION, sig);
    req.set_header("x-amz-content-sha256", hash("".to_string()));
    req.set_header("x-amz-date", now.format("%Y%m%dT%H%M%SZ").to_string());
}

#[cfg(not(feature = "auth"))]
// Stub for when authentication feature is disabled
fn set_authentication_headers(_: &mut Request) {}