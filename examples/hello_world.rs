extern crate tokio;
extern crate tokio_minihttp as http;
extern crate futures;
extern crate env_logger;

use tokio::Service;
use futures::{Finished};
use std::io;

#[derive(Clone)]
struct HelloWorld;

impl Service for HelloWorld {
    type Req = http::Request;
    type Resp = http::Response;
    type Error = io::Error;
    type Fut = Finished<http::Response, io::Error>;

    fn call(&self, _request: http::Request) -> Self::Fut {
        let resp = http::Response::new();
        futures::finished(resp)
    }
}

pub fn main() {
    let _ = ::env_logger::init();

    let addr = "0.0.0.0:8080".parse().unwrap();

    http::serve(addr, HelloWorld);
}
