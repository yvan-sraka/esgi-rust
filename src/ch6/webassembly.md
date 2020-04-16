# WebAssembly

<!-- Hello everyone, -->

## Do you hear about WASM? (<https://webassembly.org>)

- Languages "for programmers" : JavaScript / Rust
- Languages "for machines" (VMs) : WebAssembly (`.wasm`)

> Yuck! _"transpilation"_ ... JavaScript (ES2020) _turn by Babel magic into_ JavaScript (ES3 compatible)

WebAssembly: `wasmtime` (sandboxing) work with **WASI**: the WebAssembly System Interface

## Lvl 0

- Install `wasm-pack` (to compile your Rust lib in `.wasm` and more) <https://rustwasm.github.io/docs/wasm-pack/>
- Install `wasmtime` (to run a `.wasm` binary program, or `.wat` text format) <https://wasmtime.dev>

```shell
$ cargo new hello-world
$ cd hello-world
$ rustup target add wasm32-wasi
$ rustc hello.rs --target wasm32-wasi
$ wasmtime target/wasm32-wasi/debug/hello-world.wasm
Hello, world!
```

To learn more about WASI:

* <https://github.com/bytecodealliance/cargo-wasi/>
* <https://github.com/bytecodealliance/wasmtime-demos>

Or try an alternative runtime <https://wasmer.io>

## Lvl 1

We write a small program that turns Markdown (<https://commonmark.org>) into HTML as describe in the video <https://youtu.be/Qn_4F3foB3Q>

We need to add few dependencies 

* <https://crates.io/crates/pulldown-cmark>
* <https://docs.rs/wasm-bindgen/>

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"
pulldown-cmark = "0.7.0"
```

```rust
{{#include markdown.rs}}
```

Test using:

```shell
$ wasmtime target/wasm32-wasi/debug/markdown.wasm --invoke render "\# Title"
```

## Lvl 2

Try to put our code on a webpage using:

* <https://rustwasm.github.io/docs/wasm-bindgen/examples/dom.html>
* <https://rustwasm.github.io/docs/wasm-pack/tutorials/hybrid-applications-with-webpack/using-your-library.html>

The result is here: <https://github.com/yvan-sraka/markdown-viewer/blob/master/src/lib.rs>

## Lvl 3

Writing a Brainfuck Interpreter in Rust (and WebAssembly):

- Article <https://dev.to/shritesh/writing-a-brainfuck-interpreter-in-rust-and-webassembly-13f>
- Demo <https://shritesh.github.io/brainfuck-rs-wasm>
- Code <https://github.com/shritesh/brainfuck-rs-wasm>

## Lvl 4

There is an awesome Tutorial "Conway's Game of Life" on the official guide:

- <https://rustwasm.github.io/docs/book/>
- <https://xkcd.com/2293/>

## To go further

Binary Toolkit (useful for debugging): <https://github.com/WebAssembly/wabt>

Play also with <https://github.com/WebAssembly/binaryen>

Mozilla Hacks cool tech blog <https://hacks.mozilla.org/category/webassembly/>

* <https://hacks.mozilla.org/2018/04/hello-wasm-pack/>
* <https://hacks.mozilla.org/2019/08/webassembly-interface-types/>
* <https://hacks.mozilla.org/2019/11/multi-value-all-the-wasm/>
* <https://hacks.mozilla.org/2019/03/standardizing-wasi-a-webassembly-system-interface/>
* <https://hacks.mozilla.org/2017/02/a-cartoon-intro-to-webassembly/>

Another way to solve _" the JavaScript issue "_:

- **BinAST** <https://github.com/tc39/proposal-binary-ast/> & <https://github.com/binast>
- **Prepack** <https://prepack.io/>

Can we compile JS to WASM?

- <https://github.com/MichaReiser/speedy.js>
- <https://docs.assemblyscript.org>

Some JS / WASM engines:

- <https://v8.dev>
- <https://developer.mozilla.org/en-US/docs/Mozilla/Projects/SpiderMonkey>

...to not be confounded with web render engines like:

- <https://servo.org/>
- <https://webkit.org>

Happy COVID-19 quarantine, [no easter eggs](https://www.youtube.com/watch?v=dQw4w9WgXcQ) this year ...

Best, Yvan