#![allow(dead_code, unused_variables)]

use tokio::io::{Parse};
use tokio::proto::pipeline::Frame;
use bytes::{Bytes, BlockBuf};
use std::{io, slice, str};

use httparse;

pub struct Request {
    method: Slice,
    path: Slice,
    version: u8,
    // TODO: use a small vec to avoid this unconditional allocation
    headers: Vec<(Slice, Slice)>,
    data: Option<Bytes>,
}

type Slice = (usize, usize);

pub struct RequestHeaders<'req> {
    headers: slice::Iter<'req, (Slice, Slice)>,
    req: &'req Request,
}

impl Request {
    pub fn method(&self) -> &str {
        unimplemented!();
    }

    pub fn path(&self) -> &str {
        unimplemented!();
    }

    pub fn version(&self) -> u8 {
        unimplemented!();
    }

    pub fn headers(&self) -> RequestHeaders {
        unimplemented!();
    }
}

pub struct Parser;

impl Parse for Parser {
    type Out = Frame<Request, io::Error>;

    fn parse(&mut self, buf: &mut BlockBuf) -> Option<Frame<Request, io::Error>> {
        // Only compact if needed
        if !buf.is_compact() {
            buf.compact();
        }

        let mut n = 0;

        let res = {
            // TODO: we should grow this headers array if parsing fails and asks for
            //       more headers
            let mut headers = [httparse::EMPTY_HEADER; 16];
            let mut r = httparse::Request::new(&mut headers);
            let status = match r.parse(buf.bytes().expect("buffer not compact")) {
                Ok(status) => status,
                Err(e) => {
                    return Some(Frame::Error(io::Error::new(io::ErrorKind::Other,
                                                   format!("failed to parse http request: {:?}", e))));
                }
            };
            let toslice = |a: &[u8]| {
                let start = a.as_ptr() as usize - buf.bytes().expect("buffer not compact").as_ptr() as usize;
                assert!(start < buf.len());
                (start, start + a.len())
            };

            match status {
                httparse::Status::Complete(amt) => {
                    n = amt;

                    Some(Frame::Message(Request {
                        method: toslice(r.method.unwrap().as_bytes()),
                        path: toslice(r.path.unwrap().as_bytes()),
                        version: r.version.unwrap(),
                        headers: r.headers
                            .iter()
                            .map(|h| (toslice(h.name.as_bytes()), toslice(h.value)))
                            .collect(),
                        data: None,
                    }))
                }
                httparse::Status::Partial => None
            }
        };

        match res {
            Some(Frame::Message(mut msg)) => {
                msg.data = Some(buf.shift(n));
                Some(Frame::Message(msg))
            }
            res => res,
        }
    }

    fn done(&mut self, buf: &mut BlockBuf) -> Option<Frame<Request, io::Error>> {
        Some(Frame::Done)
    }
}

impl<'req> Iterator for RequestHeaders<'req> {
    type Item = (&'req str, &'req [u8]);

    fn next(&mut self) -> Option<(&'req str, &'req [u8])> {
        self.headers.next().map(|&(ref a, ref b)| {
            unimplemented!();
            /*
            let a = str::from_utf8(self.req.slice(a)).unwrap();
            let b = self.req.slice(b);
            (a, b)
            */
        })
    }
}
