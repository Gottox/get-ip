use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::server::conn::AddrStream;
use std::net::IpAddr;


fn format_ip(ip: IpAddr) -> String {
    let mut ip = match ip {
        IpAddr::V4(v4) => v4.to_string(),
        IpAddr::V6(v6) => if let Some(v4) = v6.to_ipv4() {
            v4.to_string()
        } else {
            v6.to_string()
        }
    };
    ip.push_str("\n");
    ip
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        async move {
            Ok::<_, Infallible>(service_fn(move |_: Request<Body>| async move {
                Ok::<_, Infallible>(
                    Response::new(Body::from(format_ip(remote_addr.ip())))
                    )
            }))
        }
    });

    let addr = ([0, 0, 0, 0, 0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}