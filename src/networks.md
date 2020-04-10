# Networking

Hello everyone, today for this last course, we will play with something fun:

## Lvl 0 - Hello World!

Use tokio: <https://github.com/tokio-rs/tokio> to create a small Rust program `alice` that asynchronously wait and print a message sent by a program `bob` using the standard library.

```toml
[dependencies]
tokio = { version = "0.2", features = ["full"] }
futures = "0.3"
```

```rust
{{#include alice.rs}}
```

```rust
{{#include bob.rs}}
```

**N.B.** we can use https://ngrok.com/docs or a VPN to test this code over network meanwhile social distancing ...

> Learn more about `async`: <https://areweasyncyet.rs> (you can use futures, mio or async-std)

```rust
{{#include async-await-example.rs}}
```

## Lvl 1 - HTTP server

Create a simple HTTP server using <https://hyper.rs>, that display a fancy "It's works!" message in your favorite browser on <http://127.0.0.1:8080> like shown in <https://rust-lang.github.io/async-book/01_getting_started/05_http_server_example.html>

```toml
[dependencies]
# Hyper is an asynchronous HTTP library. We'll use it to power our HTTP
# server and to make HTTP requests.
hyper = "0.13.0"
# To setup some sort of runtime needed by Hyper, we will use the Tokio runtime.
tokio = { version = "0.2", features = ["full"] }
```

```rust
{{#include mini-http-server.rs}}
```

**N.B.** for stuffs that never fail<https://doc.rust-lang.org/std/convert/enum.Infallible.html>

> There is plenty of alternatives here too: <https://www.arewewebyet.org>

## Lvl 2 - API Request

Using <https://docs.rs/reqwest> play with some external API, like Giphy one: <https://developers.giphy.com/docs/api/endpoint#random> and create a small command-line program that takes a parameter and save the GIF image on your computer!

```toml
[dependencies]
reqwest = { version = "0.10", features = ["blocking"] }
```

```rust
{{#include mini-http-client.rs}}
```

**N.B.** I use <https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html> to not write my secret `GIPHY_API_KEY` in code!

## Lvl ? - Bonus

Want to play with some "real" web framework, you can try <https://gotham.rs> (or <http://nickel-org.github.io>)!

## To go further .. are we yet?

- <https://arewegameyet.com>
- <https://areweguiyet.com>
- <https://areweaudioyet.com>
- <http://arewelearningyet.com>

## Cool stuffs for the end!!

* <https://www.wireguard.com>
* <https://github.com/ixy-languages/ixy-languages>
* <https://fr.wikipedia.org/wiki/Multipath_TCP>
* <https://etcd.io> based on <https://raft.github.io>
