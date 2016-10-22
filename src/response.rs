use std::fmt::{self, Write};

use tokio_core::easy::Serialize;

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
    type In = Response;

    fn serialize(&mut self, msg: Response, buf: &mut Vec<u8>) {
        write!(FastWrite(buf), "\
            HTTP/1.1 200 OK\r\n\
            Server: Example\r\n\
            Content-Length: {}\r\n\
            Date: {}\r\n\
        ", msg.response.len(), ::date::now()).unwrap();

        for &(ref k, ref v) in &msg.headers {
            buf.extend_from_slice(k.as_bytes());
            buf.extend_from_slice(b": ");
            buf.extend_from_slice(v.as_bytes());
            buf.extend_from_slice(b"\r\n");
        }

        buf.extend_from_slice(b"\r\n");
        buf.extend_from_slice(msg.response.as_bytes());
    }
}

// TODO: impl fmt::Write for Vec<u8>
//
// Right now `write!` on `Vec<u8>` goes through io::Write and is not super
// speedy, so inline a less-crufty implementation here which doesn't go through
// io::Error.
struct FastWrite<'a>(&'a mut Vec<u8>);

impl<'a> fmt::Write for FastWrite<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        fmt::write(self, args)
    }
}
