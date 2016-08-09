# Tokio MiniHTTP

This library is a proof-of-concept implementation of an HTTP server using Tokio.

The goal of the library is to demo the simplicity of implementing a
protocol with Tokio. This is part of an effort of experimenting with
multiple IO strategies in Rust in order to try to figure out the best
path forward.

This implementation of HTTP, while far from complete, demonstrates:

* It is very simple to implement a complex protocol using Tokio.
* It is very fast.

Below are the results of running the futures-minihttp benchmarkes on my
machine (an older macbook pro):

|   program                     | pipelined    | singlethread, no pipeline |
|-------------------------------|-------------:|--------------------------:|
| Tokio MiniHTP                 |   526,087.83 |                 56,049.39 |
| futures-minihttp              |   500,142.46 |                 55,261.12 |

The units are requests per second.
