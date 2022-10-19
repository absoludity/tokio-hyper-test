use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use log::info;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};

const DEFAULT_SLEEP_MILLIS: u64 = 30;
// Requests to this downstream just delay a set number of milliseconds,
// defaulting to 30
async fn delayed_response(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let millis = match (req.method(), req.uri().path()) {
        // (&Method::GET, "/30") => 30,
        // (&Method::GET, "/60") => 60,
        // (&Method::GET, "/300") => 300,
        // (&Method::GET, "/3000") => 3000,
        _ => DEFAULT_SLEEP_MILLIS,
    };
    sleep(Duration::from_millis(millis)).await;
    info!("{} {}", req.method(), req.uri());
    Ok(Response::new(
        format!("Hello, World - *yawn* - slept for {}ms\n", millis).into(),
    ))
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(delayed_response)) });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Starting server on {}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
