extern crate env_logger;
extern crate futures;
extern crate num_cpus;
extern crate rustc_serialize;
extern crate tokio_minihttp;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;

use futures::future;
use tokio_minihttp::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;

struct Techempower1;

#[derive(RustcEncodable)]
struct Message {
    message: String,
}

impl Service for Techempower1 {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>;

    fn call(&self, request: Request) -> Self::Future {
        assert_eq!(request.path(), "/json");
        let msg = Message { message: "Hello, World!".to_string() };
        let mut r = Response::new();
        r.header("Content-Type", "application/json")
         .body(&rustc_serialize::json::encode(&msg).unwrap());
        future::ok(r)
    }
}

fn main() {
    drop(env_logger::init());
    let addr = "0.0.0.0:8080".parse().unwrap();
    let mut srv = TcpServer::new(Http, addr);
    srv.threads(num_cpus::get());
    srv.serve(|| Ok(Techempower1));
}
