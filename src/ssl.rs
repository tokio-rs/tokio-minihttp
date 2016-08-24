use tokio::io::{Stream, Readiness};
use tokio_ssl::{SslStream, ServerContext};
use std::{io, mem};

pub trait NewSslContext: Send + 'static {
    fn new_context(&self) -> io::Result<ServerContext>;
}

// TODO: Figure out how to build stacks better
pub enum MaybeSsl<S> {
    Ssl(SslStream<S>),
    NoSsl(S),
    Invalid,
}

impl<S: Stream> MaybeSsl<S> {
    pub fn new(s: S) -> MaybeSsl<S> {
        MaybeSsl::NoSsl(s)
    }

    pub fn establish(&mut self, context: ServerContext) {
        match mem::replace(self, MaybeSsl::Invalid) {
            MaybeSsl::NoSsl(stream) => {
                *self = MaybeSsl::Ssl(context.establish(stream));
            }
            _ => panic!("invalid state"),
        }
    }
}

impl<S: Stream> io::Read for MaybeSsl<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            MaybeSsl::Ssl(ref mut s) => s.read(buf),
            MaybeSsl::NoSsl(ref mut s) => s.read(buf),
            _ => panic!("invalid state"),
        }
    }
}

impl<S: Stream> io::Write for MaybeSsl<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            MaybeSsl::Ssl(ref mut s) => s.write(buf),
            MaybeSsl::NoSsl(ref mut s) => s.write(buf),
            _ => panic!("invalid state"),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            MaybeSsl::Ssl(ref mut s) => s.flush(),
            MaybeSsl::NoSsl(ref mut s) => s.flush(),
            _ => panic!("invalid state"),
        }
    }
}

impl<S: Stream> Readiness for MaybeSsl<S> {
    fn is_readable(&self) -> bool {
        match *self {
            MaybeSsl::Ssl(ref s) => s.is_readable(),
            MaybeSsl::NoSsl(ref s) => s.is_readable(),
            _ => panic!("invalid state"),
        }
    }

    fn is_writable(&self) -> bool {
        match *self {
            MaybeSsl::Ssl(ref s) => s.is_writable(),
            MaybeSsl::NoSsl(ref s) => s.is_writable(),
            _ => panic!("invalid state"),
        }
    }
}

impl<F> NewSslContext for F
    where F: Fn() -> io::Result<ServerContext> + Send + 'static,
{
    fn new_context(&self) -> io::Result<ServerContext> {
        self()
    }
}
