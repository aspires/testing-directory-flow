//! Default Compute@Edge template program.

use fastly::http::{HeaderValue, Method, StatusCode};
use fastly::request::CacheOverride;
use fastly::{Body, Error, Request, RequestExt, Response, ResponseExt};

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND_NAME: &str = "backend_name";

/// The name of a second backend associated with this service.
const OTHER_BACKEND_NAME: &str = "other_backend_name";

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(mut req: Request<Body>) -> Result<impl ResponseExt, Error> {
    // Make any desired changes to the client request.
    req.headers_mut()
        .insert("Host", HeaderValue::from_static("example.com"));

    // We can filter requests that have unexpected methods.
    const VALID_METHODS: [Method; 3] = [Method::HEAD, Method::GET, Method::POST];
    if !(VALID_METHODS.contains(req.method())) {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::from("This method is not allowed"))?);
    }

    // Pattern match on the request method and path.
    match (req.method(), req.uri().path()) {
        // If request is a `GET` to the `/` path, send a default response.
        (&Method::GET, "/") => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Welcome to Fastly Compute@Edge!"))?),

        // If request is a `GET` to the `/backend` path, send to a named backend.
        (&Method::GET, "/backend") => {
            // Request handling logic could go here...
            // E.g., send the request to an origin backend and then cache the
            // response for one minute.
            *req.cache_override_mut() = CacheOverride::ttl(60);
            Ok(req.send(BACKEND_NAME)?)
        }

        // If request is a `GET` to a path starting with `/other/`.
        (&Method::GET, path) if path.starts_with("/other/") => {
            // Send request to a different backend and don't cache response.
            *req.cache_override_mut() = CacheOverride::Pass;
            Ok(req.send(OTHER_BACKEND_NAME)?)
        }

        (&Method::GET, path) if path.starts_with("/headers/") => {
            // Demo adding, editing, and deleting headers
            
            // Code examples go here
            // Suitable for breaking into subpaths (/headers/edit etc)
        }

        (&Method::GET, path) if path.starts_with("/geoip/") => {
            // Demo of a simple geoIP function
            
            // Code examples go here
            // link to external resources go here
        }

        (&Method::GET, path) if path.starts_with("/logging/") => {
            // Demo of a streaming logs
            
            // Code examples go here
            // link to external resources go here        
        }

        (&Method::GET, path) if path.starts_with("/error/") => {
            // Demo of a error-handling functions. Subdirectory paths likely needed
            
            // Code examples go here
            // link to external resources go here        
        }

        (&Method::GET, path) if path.starts_with("/http/") => {
            // Demo of a http functions. Subdirectory paths likely needed
            
            // Code examples go here
            // link to external resources go here        
        }

        (&Method::GET, path) if path.starts_with("/url/") => {
            // Demo of a url editiing.
            
            // Code examples go here
            // link to external resources go here        
        }

        (&Method::POST, path) if path.starts_with("/post/") => {
            // Demo of a POST method handling.
            
            // Code examples go here
            // link to external resources go here        
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("The page you requested could not be found"))?),
    }
}
