extern crate tokio;
extern crate tokio_minihttp as http;
extern crate tokio_ssl as ssl;
extern crate openssl;
extern crate futures;
extern crate env_logger;

use tokio::Service;
use futures::{Finished};
use openssl::crypto::pkey::PKey;
use openssl::x509::X509;
use std::io;
use std::net::SocketAddr;
use std::path::Path;

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

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();

    println!("\n\
        Accepting connections on {addr}\n\n\
        to connect to the server run:\n\n    \
            curl --cacert {path} https://localhost:{port}/plaintext\n\n\
    ",
        addr = addr,
        port = addr.port(),
        path = Path::new(file!()).parent().unwrap().join("server.crt")
                     .display(),
    );

    http::Server::new(addr)
        .ssl(move || {
            macro_rules! t {
                ($e:expr) => (match $e {
                    Ok(e) => e,
                    Err(e) => panic!("{} failed with {:?}", stringify!($e), e),
                })
            }

            let cert = include_bytes!("server.crt");
            let cert = t!(X509::from_pem(&mut &cert[..]));
            let key = include_bytes!("server.key");
            let key = t!(PKey::private_key_from_pem(&mut &key[..]));

            ssl::ServerContext::new(&cert, &key)
        })
        .serve(HelloWorld);
}
