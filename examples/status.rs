extern crate tokio_service;
extern crate tokio_minihttp as http;
extern crate futures;
extern crate env_logger;

use tokio_service::Service;
use futures::{Async, Finished};
use std::io;

#[derive(Clone)]
struct StatusService;

impl Service for StatusService {
    type Request = http::Request;
    type Response = http::Response;
    type Error = io::Error;
    type Future = Finished<http::Response, io::Error>;

    fn call(&self, _request: http::Request) -> Self::Future {
        let (code, message) = match _request.path() {
            "/200" => (200, "OK"),
            "/400" => (400, "Bad Request"),
            "/500" => (500, "Internal Server Error"),
            _ => (404, "Not Found")
        };

        let mut resp = http::Response::new();
        resp.status(code, message);
        resp.body(message);
        futures::finished(resp)
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

fn main() {
    drop(env_logger::init());
    let addr = "0.0.0.0:8080".parse().unwrap();
    http::Server::new(addr).serve(StatusService);
}
