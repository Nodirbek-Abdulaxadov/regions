use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use serde::{Serialize, Deserialize};
use std::convert::Infallible;

#[derive(Debug, Serialize, Deserialize)]
struct District {
    id: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Region {
    id: u32,
    name: String,
    districts: Vec<District>,
}

async fn handle_uz_uz_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Read the file content as bytes
    let file_content = match tokio::fs::read("regions_uz_Uz.json").await {
        Ok(content) => content,
        Err(_) => {
            let mut response = Response::new(Body::from("Failed to open file"));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(response);
        }
    };

    // Convert bytes to UTF-8 string (assumes the file content is UTF-8 encoded)
    let content_str = match String::from_utf8(file_content) {
        Ok(content) => content,
        Err(_) => {
            let mut response = Response::new(Body::from("Failed to parse UTF-8"));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(response);
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(content_str))
        .unwrap())
}

async fn handle_uz_kr_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Read the file content as bytes
    let file_content = match tokio::fs::read("regions_uz_Kr.json").await {
        Ok(content) => content,
        Err(_) => {
            let mut response = Response::new(Body::from("Failed to open file"));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(response);
        }
    };

    // Convert bytes to UTF-8 string (assumes the file content is UTF-8 encoded)
    let content_str = match String::from_utf8(file_content) {
        Ok(content) => content,
        Err(_) => {
            let mut response = Response::new(Body::from("Failed to parse UTF-8"));
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            return Ok(response);
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(content_str))
        .unwrap())
}

async fn run_server() -> Result<(), hyper::Error> {
    let make_svc = make_service_fn(|_conn| {
        async {
            Ok::<_, Infallible>(service_fn(|req| {
                async {
                    match (req.method(), req.uri().path()) {
                        (&hyper::Method::GET, "/uz_Uz") => handle_uz_uz_request(req).await,
                        (&hyper::Method::GET, "/uz_Kr") => handle_uz_kr_request(req).await,
                        _ => {
                            // Return a 404 Not Found response for unknown routes
                            let response = Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::from("Not Found"))
                                .unwrap();
                            Ok(response)
                        }
                    }
                }
            }))
        }
    });

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run_server().await {
        eprintln!("server error: {}", e);
    }
}