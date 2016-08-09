extern crate tokio;
extern crate bytes;
extern crate time;
extern crate httparse;

mod date;
mod request;
mod response;

pub use request::Request;
pub use response::Response;

use tokio::{server, NewService};
use tokio::io::Framed;
use tokio::proto::pipeline::Server;
use tokio::reactor::Reactor;
use bytes::BlockBuf;
use std::io;
use std::net::SocketAddr;

pub fn serve<T>(addr: SocketAddr, new_service: T)
    where T: NewService< Req = Request, Resp = Response, Error = io::Error> + Send + 'static {

    let reactor = Reactor::default().unwrap();
    let handle = reactor.handle();

    server::listen(&handle, addr, move |socket| {
        // Create the service
        let service = try!(new_service.new_service());

        // Create the transport
        let transport =
            Framed::new(socket,
                        request::Parser,
                        response::Serializer,
                        BlockBuf::default(),
                        BlockBuf::default());

        // Return the pipeline server task
        Server::new(service, transport)
    }).unwrap();

    reactor.run().unwrap();
}
