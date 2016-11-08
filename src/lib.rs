extern crate futures;
extern crate httparse;
extern crate net2;
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
use std::sync::Arc;
use std::thread;

use futures::stream::Stream;
use futures::Future;
use tokio_core::reactor::{Core, Handle};
use tokio_core::easy::EasyFramed;
use tokio_core::net::TcpListener;
use tokio_proto::easy::pipeline;
use tokio_service::NewService;

pub struct Server {
    addr: SocketAddr,
    threads: usize,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Server {
            threads: 1,
            addr: addr,
        }
    }

    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.addr = addr;
        self
    }

    pub fn threads(mut self, threads: usize) -> Self {
        assert!(threads > 0);
        if cfg!(unix) {
            self.threads = threads;
        }
        self
    }

    pub fn serve<T>(self, new_service: T)
        where T: NewService<Request = Request,
                            Response = Response,
                            Error = io::Error> + Send + Sync + 'static,
    {
        let new_service = Arc::new(new_service);
        let addr = self.addr;
        let workers = self.threads;

        let threads = (0..self.threads - 1).map(|i| {
            let new_service = new_service.clone();
            thread::Builder::new().name(format!("worker{}", i)).spawn(move || {
                serve(addr, workers, &new_service)
            }).unwrap()
        }).collect::<Vec<_>>();

        serve(addr, workers, &new_service);

        for thread in threads {
            thread.join().unwrap();
        }
    }
}

fn serve<T>(addr: SocketAddr, workers: usize, new_service: &Arc<T>)
    where T: NewService<Request = Request,
                        Response = Response,
                        Error = io::Error> + 'static,
{
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let listener = listener(&addr, workers, &handle).unwrap();
    let server = listener.incoming().for_each(move |(socket, _)| {
        // Create the service
        let service = try!(new_service.new_service());

        // Create the transport
        let transport = EasyFramed::new(socket,
                                        request::Decoder,
                                        response::Encoder);

        // Return the pipeline server task
        let server = pipeline::EasyServer::new(service, transport);
        handle.spawn(server.map_err(|e| {
            println!("error: {}", e);
        }));
        Ok(())
    });
    core.run(server).unwrap();
}

fn listener(addr: &SocketAddr,
            workers: usize,
            handle: &Handle) -> io::Result<TcpListener> {
    let listener = try!(net2::TcpBuilder::new_v4());
    try!(configure_tcp(workers, &listener));
    try!(listener.reuse_address(true));
    try!(listener.bind(addr));
    listener.listen(1024).and_then(|l| {
        TcpListener::from_listener(l, addr, handle)
    })
}

#[cfg(unix)]
fn configure_tcp(workers: usize, tcp: &net2::TcpBuilder) -> io::Result<()> {
    use net2::unix::*;

    if workers > 1 {
        try!(tcp.reuse_port(true));
    }

    Ok(())
}

#[cfg(windows)]
fn configure_tcp(workers: usize, _tcp: &net2::TcpBuilder) -> io::Result<()> {
    Ok(())
}
