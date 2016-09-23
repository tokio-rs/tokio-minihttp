use std::fmt::Write;

use tokio_proto::Serialize;
use tokio_proto::pipeline::Frame;
use bytes::MutBuf;
use bytes::buf::{BlockBuf, Fmt};
use std::io;

pub struct Response {
    headers: Vec<(String, String)>,
    response: String,
}

pub struct Serializer;

impl Response {
    pub fn new() -> Response {
        Response {
            headers: Vec::new(),
            response: String::new(),
        }
    }

    pub fn header(&mut self, name: &str, val: &str) -> &mut Response {
        self.headers.push((name.to_string(), val.to_string()));
        self
    }

    pub fn body(&mut self, s: &str) -> &mut Response {
        self.response = s.to_string();
        self
    }
}

impl Serialize for Serializer {
    type In = Frame<Response, (), io::Error>;

    fn serialize(&mut self, frame: Frame<Response, (), io::Error>, buf: &mut BlockBuf) {
        match frame {
            Frame::Message(msg) => {
                write!(Fmt(buf), "\
                    HTTP/1.1 200 OK\r\n\
                    Server: Example\r\n\
                    Content-Length: {}\r\n\
                    Date: {}\r\n\
                ", msg.response.len(), ::date::now()).unwrap();

                for &(ref k, ref v) in &msg.headers {
                    buf.copy_from(k.as_bytes());
                    buf.write_slice(b": ");
                    buf.copy_from(v.as_bytes());
                    buf.write_slice(b"\r\n");
                }

                buf.write_slice(b"\r\n");
                buf.copy_from(msg.response.as_bytes());
            }
            _ => {},
        }
    }
}
