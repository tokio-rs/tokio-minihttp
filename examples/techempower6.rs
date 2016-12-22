extern crate env_logger;
extern crate futures;
extern crate num_cpus;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;

use futures::future;
use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

struct Techempower6;

impl Service for Techempower6 {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&mut self, _request: Request) -> Self::Future {
        let mut resp = Response::new();
        resp.header("Content-Type", "text/plain")
            .body("Hello, World!");
        future::ok(resp)
    }
}

fn main() {
    drop(env_logger::init());
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut srv = TcpServer::new(Http, addr);
    srv.threads(num_cpus::get());
    srv.serve(|| Ok(Techempower6));
}
