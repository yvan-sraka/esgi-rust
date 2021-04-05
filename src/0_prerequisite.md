# Prerequisite

## 1. Get a [Rust](https://www.rust-lang.org)

### Using Windows

Don't.

More seriously I highly recommend [the installation of a WSL](https://docs.microsoft.com/en-us/windows/wsl/install-win10) (Windows Subsystem for Linux) and [this nice VSCode extension](https://code.visualstudio.com/remote-tutorials/wsl/run-in-wsl). Then follow the UNIX instructions behind (inside the `bash.exe` shell).

> ⚠️ `rustc` needs `cc` linker (included in C/C++ toolchain) on Debian-based linux run:
>
> ```shell
> sudo apt update && sudo apt install build-essential
> ```

### Using UNIX (e.g. macOS or Linux)

We will use [rustup](https://rustup.rs), run the following in your terminal, then follow the onscreen instructions.

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 2. Get an IDE

Rust offers good [first-class editor support](https://www.rust-lang.org/tools) with [RLS](https://github.com/rust-lang/rls) (Rust Language Server). If you don't have yet a preferred IDE, pick VSCode (Visual Studio Code).

### Using [VSCode](https://code.visualstudio.com)

Install the [RLS Extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) and run the shell command:

```shell
rustup component add rls rust-analysis rust-src
```

## Congratulations 🎉

You're ready to hack with Rust 🦀 !

In the first session, we will get familiar with language syntax by playing a bit with [rustlings](https://github.com/rust-lang/rustlings) tutorial.
