extern crate tokio_service;
extern crate tokio_minihttp as http;
extern crate futures;
extern crate env_logger;

use tokio_service::Service;
use futures::{Async, Finished};
use std::io;

/// `HelloWorld` is the *service* that we're going to be implementing to service
/// the HTTP requests we receive.
///
/// The tokio-minihttp crate, and much of Tokio itself, are centered around the
/// concept of a service for interoperability between crates. Our service here
/// carries no data with it, and we implement `Clone` to satisfy the
/// `NewService` bound we'll need to start serving requests.
///
/// Note that a new instance of `HelloWorld` is created for each TCP connection
/// we service, in this case they're all just clones of the first one we create.
#[derive(Clone)]
struct HelloWorld;

impl Service for HelloWorld {
    type Request = http::Request;
    type Response = http::Response;
    type Error = io::Error;
    type Future = Finished<http::Response, io::Error>;

    fn call(&self, _request: http::Request) -> Self::Future {
        let mut resp = http::Response::new();
        resp.body("Hello, world!");
        futures::finished(resp)
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

fn main() {
    drop(env_logger::init());
    let addr = "0.0.0.0:8080".parse().unwrap();
    http::Server::new(addr)
        .serve(HelloWorld);
}
