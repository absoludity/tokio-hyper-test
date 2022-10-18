use chrono::Local;
use std::convert::Infallible;
use std::io::Write;
use std::net::SocketAddr;
use hyper::{Client, Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use log::{info, Level};
use env_logger::fmt::Color;

const DOWNSTREAM_URI: &str = "http://127.0.0.1:3030";

async fn proxy(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let client = Client::new();
    let proxy_uri = format!("{}{}", DOWNSTREAM_URI, req.uri().path());
    let req_proxy = Request::builder()
        .uri(proxy_uri.clone())
        .method(req.method())
        .body(Body::from("")).unwrap();

    info!("Start proxy of {} {} -> {}", req.method(), req.uri(), req_proxy.uri());
    let response = client.request(req_proxy).await.or_else(|r| {
        let response = Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Error proxying request {}", r).into()).unwrap();
        Ok(response)
    });
    info!("Finished proxy of {}", proxy_uri);
    response
}

fn setup_logger() {
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let level = record.level();
            let mut level_style = buf.style();

            match level {
                Level::Trace => level_style.set_color(Color::White),
                Level::Debug => level_style.set_color(Color::Blue),
                Level::Info => level_style.set_color(Color::Green),
                Level::Warn => level_style.set_color(Color::Yellow),
                Level::Error => level_style.set_color(Color::Red).set_bold(true),
            };

            let level_ok = write!(buf, "{:>5} ", level_style.value(level));

            let ts = Local::now().format("%Y-%m-%dT%H:%M:%S%.6f");
            let ts_ok = write!(buf, "{}: ", ts);

            let module_ok = if let Some(module_path) = record.module_path() {
                write!(buf, "{}: ", module_path)
            } else {
                Ok(())
            };

            let args_ok = writeln!(buf, "{}", record.args());
            level_ok.and(ts_ok).and(module_ok).and(args_ok)
        })
        .init();
}

#[tokio::main]
async fn main() {
    setup_logger();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(proxy))
    });

    let server = Server::bind(&addr).serve(make_svc);

    info!("Starting server on {}", addr);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
