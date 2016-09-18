extern crate bytes;
extern crate futures;
extern crate httparse;
extern crate time;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

mod date;
mod request;
mod response;

pub use request::Request;
pub use response::Response;

use std::io;
use std::net::SocketAddr;

use bytes::BlockBuf;
use futures::stream::Receiver;
use futures::{Async, Future, Map};
use tokio_core::reactor::Core;
use tokio_proto::Framed;
use tokio_proto::{pipeline, server};
use tokio_service::{Service, NewService};

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Server {
            addr: addr,
        }
    }

    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.addr = addr;
        self
    }

    pub fn serve<T>(self, new_service: T)
        where T: NewService<Request = Request, Response = Response, Error = io::Error> + Send + 'static
    {
        let mut lp = Core::new().unwrap();
        let addr = self.addr;

        server::listen(&lp.handle(), addr, move |socket| {
            // Create the service
            let service = try!(new_service.new_service());
            let service = HttpService { inner: service };

            // Create the transport
            let transport =
                Framed::new(socket,
                            request::Parser,
                            response::Serializer,
                            BlockBuf::default(),
                            BlockBuf::default());

            // Return the pipeline server task
            pipeline::Server::new(service, transport)
        }).unwrap();
        lp.run(futures::empty::<(), ()>()).unwrap();
    }
}

impl Default for Server {
    fn default() -> Server {
        Server {
            addr: "0.0.0.0:3000".parse().unwrap(),
        }
    }
}

struct HttpService<T> {
    inner: T,
}

impl<T> Service for HttpService<T>
    where T: Service<Request = Request, Response = Response, Error = io::Error>,
{
    type Request = Request;
    type Response = pipeline::Message<Response, Receiver<(), io::Error>>;
    type Error = io::Error;
    type Future = Map<T::Future, fn(Response) -> pipeline::Message<Response, Receiver<(), io::Error>>>;

    fn call(&self, req: Request) -> Self::Future {
        self.inner.call(req).map(pipeline::Message::WithoutBody)
    }

    fn poll_ready(&self) -> Async<()> {
        // TODO: Don't always return ready
        Async::Ready(())
    }
}

pub fn serve<T>(addr: SocketAddr, new_service: T)
    where T: NewService<Request = Request, Response = Response, Error = io::Error> + Send + 'static
{
    Server::default()
        .addr(addr)
        .serve(new_service)
}
