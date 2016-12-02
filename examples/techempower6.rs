extern crate env_logger;
extern crate futures;
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

    fn call(&self, _request: Request) -> Self::Future {
        let mut resp = Response::new();
        resp.header("Content-Type", "text/plain")
            .body("Hello, World!");
        future::ok(resp)
    }
}

fn main() {
    drop(env_logger::init());
    let addr = "0.0.0.0:8080".parse().unwrap();
    TcpServer::new(Http, addr)
        .serve(|| Ok(Techempower6));
}
