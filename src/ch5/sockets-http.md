Sockets and HTTP
================

<!-- Hello everyone, -->

Lvl 0 - Hello World!
--------------------

Today we will play with something fun,
[sockets](https://en.wikipedia.org/wiki/Network_socket):

Use `tokio`: <https://github.com/tokio-rs/tokio> to create a small Rust
program `alice` that asynchronously wait and print a message sent by a
program `bob` using the standard library.

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

> **N.B.** we can use https://ngrok.com/docs or a VPN to test this code
> over network meanwhile social distancing ...

Learn more about `async`: <https://areweasyncyet.rs> (you can use
`futures`, `mio` or `async-std`)

```rust
{{#include async-await-example.rs}}
```

Lvl 1 - HTTP server
-------------------

Create a simple HTTP server using <https://hyper.rs>, that display a
fancy "It's works!" message in your favorite browser on
<http://127.0.0.1:8080> like shown in
<https://rust-lang.github.io/async-book/01_getting_started/05_http_server_example.html>

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

> **N.B.** for stuffs that never
> fail<https://doc.rust-lang.org/std/convert/enum.Infallible.html>

There is plenty of alternatives here too: <https://www.arewewebyet.org>

Lvl 2 - API Request
-------------------

Using <https://docs.rs/reqwest> play with some external API, like Giphy
one: <https://developers.giphy.com/docs/api/endpoint#random> and create
a small command-line program that takes a parameter and save the GIF
image on your computer!

```toml
[dependencies]
reqwest = { version = "0.10", features = ["blocking"] }
```

```rust
{{#include mini-http-client.rs}}
```

> **N.B.** I use
> <https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html>
> to not write my secret `GIPHY_API_KEY` in code!

Lvl ? - Bonus
-------------

Want to play with some "real" web framework, you can try
<https://gotham.rs> (or <http://nickel-org.github.io>)!

There is also a cool static site generator
(<https://cobalt-org.github.io>) or ORM (<https://diesel.rs>), and here
is a bunch of supplementary resources, if you're interested in doing
*web* stuff in Rust:

-   [Rust web framework
    comparison](https://github.com/flosse/rust-web-framework-comparison)
-   [A small Rust API with
    Actix](https://stevezeidner.com/blog/rust-actix-api/)
-   [Writing a Simple Web Service in
    Rust](https://danielwelch.github.io/rust-web-service.html)
-   [Auth Web Microservice with rust using
    Actix-Web](https://gill.net.in/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-1/)
-   [A web application completely written in
    Rust](https://github.com/saschagrunert/webapp.rs)
-   [Touring a Fast, Safe, and Complete(ish) Web Service in
    Rust](https://brandur.org/rust-web)

To go further .. are we yet?
----------------------------

-   <https://arewegameyet.com>
-   <https://areweguiyet.com>
-   <https://areweaudioyet.com>
-   <http://arewelearningyet.com>

Cool stuff for the end!!
------------------------

-   <https://www.wireguard.com>
-   <https://github.com/ixy-languages/ixy-languages>
-   <https://fr.wikipedia.org/wiki/Multipath_TCP>
-   <https://etcd.io> based on <https://raft.github.io>

Cheers, Yvan

<!--

# Sockets & HTTP

Bonjour à tous, aujourd'hui nous allons jouer avec quelque chose d'amusant, des sockets:

## Lvl 0 - Bonjour tout le monde!

Utilisez `tokio`: <https://github.com/tokio-rs/tokio> pour créer un petit programme Rust` alice` qui attend de manière asynchrone et imprime un message envoyé par un programme `bob` en utilisant la bibliothèque standard.

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

> ** N.B.** nous pouvons utiliser https://ngrok.com/docs ou un VPN pour tester ce code sur le réseau en attendant la distanciation sociale ...

En savoir plus sur `async` : <https://areweasyncyet.rs> (vous pouvez utiliser `futures`, `mio` ou `async-std`)

```rust
{{#include async-await-example.rs}}
```

## Lvl 1 - Serveur HTTP

Créez un serveur HTTP simple à l'aide de <https://hyper.rs>, qui affiche un simple message "It's works!" dans votre navigateur préféré sur <http://127.0.0.1:8080> comme indiqué dans <https://rust-lang.github.io/async-book/01_getting_started/05_http_server_example.html>

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

> ** N.B.** pour les erreurs qui n'ont jamais lieux <https://doc.rust-lang.org/std/convert/enum.Infallible.html>

Il existe également de nombreuses alternatives: <https://www.arewewebyet.org>

## Lvl 2 - Demande d'API

En utilisant <https://docs.rs/reqwest>, jouez avec une API externe, comme celle de Giphy : <https://developers.giphy.com/docs/api/endpoint#random> et créez un petit programme en ligne de commande qui prend un paramètre et enregistre l'image GIF sur votre ordinateur!

```toml
[dependencies]
reqwest = { version = "0.10", features = ["blocking"] }
```

```rust
{{#include mini-http-client.rs}}
```

> ** N.B.** J'utilise <https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html> pour ne pas écrire mon secret `GIPHY_API_KEY` dans le code !

## Niv. ???

Vous voulez jouer avec un cadre Web "réel", vous pouvez essayer <https://gotham.rs> (ou <http://nickel-org.github.io>)!

Il y a aussi des générateurs de site statique sympa (<https://cobalt-org.github.io>) ou des ORM (<https://diesel.rs>), et voici un tas de ressources supplémentaires, si vous êtes intéressé à faire des trucs _web_ avec Rust :

* [Comparaison du framework web Rust] (https://github.com/flosse/rust-web-framework-comparison)
* [Une petite API Rust avec Actix] (https://stevezeidner.com/blog/rust-actix-api/)
* [Écriture d'un service Web simple dans Rust] (https://danielwelch.github.io/rust-web-service.html)
* [Auth Web Microservice avec rouille en utilisant Actix-Web] (https://gill.net.in/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-1/)
* [Une application Web entièrement écrite en rouille] (https://github.com/saschagrunert/webapp.rs)
* [Visite guidée d'un service Web rapide, sûr et complet (ish) à Rust] (https://brandur.org/rust-web)

## Pour aller plus loin .. _are we yet?__

- <https://arewegameyet.com>
- <https://areweguiyet.com>
- <https://areweaudioyet.com>
- <http://arewelearningyet.com>

## Encore des trucs sympas pour la fin !!

* <https://www.wireguard.com>
* <https://github.com/ixy-languages/ixy-languages>
* <https://fr.wikipedia.org/wiki/Multipath_TCP>
* <https://etcd.io> basé sur <https://raft.github.io>

Amitiés, Yvan

-->