use std::convert::Infallible;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::info;
use std::net::IpAddr;

fn format_ip(req: Request<Body>, ip: IpAddr) -> String {
    let mut ip = if let Some(ip) = req.headers().get("X-Forwarded-For") {
        ip.to_str().unwrap_or("0.0.0.0").to_string()
    } else {
        match ip {
            IpAddr::V4(v4) => v4.to_string(),
            IpAddr::V6(v6) => {
                if let Some(v4) = v6.to_ipv4() {
                    v4.to_string()
                } else {
                    v6.to_string()
                }
            }
        }
    };
    info!("Request from {}", ip);
    ip.push_str("\n");
    ip
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                async move {
                    Ok::<_, Infallible>(Response::new(Body::from(format_ip(req, remote_addr.ip()))))
                }
            }))
        }
    });

    let port = std::env::args()
        .skip(1)
        .next()
        .map(|x| x.parse().expect("Cannot parse argument as port"))
        .unwrap_or(3000);

    let addr = ([0, 0, 0, 0, 0, 0, 0, 0], port).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
