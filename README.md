# Tokio MiniHTTP

[![Build Status](https://travis-ci.org/tokio-rs/tokio-minihttp.svg?branch=master)](https://travis-ci.org/tokio-rs/tokio-minihttp)
[![Build status](https://ci.appveyor.com/api/projects/status/pxh2602owjq4kn6b?svg=true)](https://ci.appveyor.com/project/alexcrichton/tokio-minihttp)

This library is a proof-of-concept implementation of a simple HTTP/1.1 server
using Tokio.

The goal of the library is to demo the simplicity of implementing a protocol
with Tokio. This is part of an effort of experimenting with multiple IO
strategies in Rust in order to try to figure out the best path forward.

This implementation of HTTP, while far from complete, demonstrates:

* It is very simple to implement a complex protocol using Tokio.
* It is very fast.
