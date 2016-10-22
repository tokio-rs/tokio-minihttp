extern crate env_logger;
extern crate futures;
extern crate num_cpus;
extern crate rustc_serialize;
extern crate tokio_minihttp as http;
extern crate tokio_service;

use tokio_service::Service;
use futures::{Async, Finished};
use std::io;

#[derive(Clone)]
struct Techempower1;

#[derive(RustcEncodable)]
struct Message {
    message: String,
}

impl Service for Techempower1 {
    type Request = http::Request;
    type Response = http::Response;
    type Error = io::Error;
    type Future = Finished<http::Response, io::Error>;

    fn call(&self, request: http::Request) -> Self::Future {
        assert_eq!(request.path(), "/json");
        let msg = Message { message: "Hello, World!".to_string() };
        let mut r = http::Response::new();
        r.header("Content-Type", "application/json")
         .body(&rustc_serialize::json::encode(&msg).unwrap());
        futures::finished(r)
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

fn main() {
    drop(env_logger::init());
    let addr = "0.0.0.0:8080".parse().unwrap();
    http::Server::new(addr)
        .threads(num_cpus::get())
        .serve(Techempower1);
}
