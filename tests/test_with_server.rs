use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use rtcps::port_scanner::PortScanner;
use std::convert::Infallible;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::thread::sleep;
use std::time::Duration;

const PORT: u16 = 63486;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World")))
}

async fn start_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));

    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_scanner_with_server() {
    tokio::spawn(start_server());
    sleep(Duration::from_secs(3));

    let port_scanner = PortScanner::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert!(port_scanner.check_port_open(&PORT, None).await);
}
