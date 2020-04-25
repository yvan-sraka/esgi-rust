FFI and unsafe
==============

<!-- Hello everyone, -->

_Lvl -1, 0, 1 recalls from previous sessions ..._

Lvl -1: Requierements (quick setup)
-----------------------------------

Have an **up to date rust toolchain**:

    rustup update

If you need a recap on **Rust syntax**:

-   <https://github.com/rust-lang/rustlings>
-   <https://learnxinyminutes.com/docs/rust/>
-   <https://exercism.io/tracks/rust>

**Tooling**: be sure to have **RLS**
(<https://github.com/rust-lang/rls>) linting enabled in your editor!

> **N.B.** if you're looking for a cool font with code ligatures:
> <https://github.com/tonsky/FiraCode>

Lvl 0: Remind me, what's Rust again?
------------------------------------

Rust is a **compiled language** with a **strong static** algebraic type
system.

**Reminder:**

-   "static" stands for "computed at compiletime", e.g. `macros`
-   "dynamic" stands for "computed at runtime", e.g. `fn`

The Rust type system contains information about memory (ownership &
lifetime).

**Rust rule of thumb:** in safe mode, we can't have aliasing AND
mutability!

### A quick borrowing example

Try to recall that memory is split between:

-   the **stack** (for things that size is known at compile time)
-   the **heap** (for everything else)

```rust
{{#include mine.rs}}
```

> **N.B.** <https://doc.rust-lang.org/std/string/struct.String.html>

Lvl 1: Sort weel-known programming languages
--------------------------------------------

### Static typing (compiled languages)

-   **Static memory management (language that compiles to arch-specific
    binaries)**: Rust (type inference is everywhere) \| C++ (type
    inference with keyword `auto`) \| C (no type inference, need to be
    explicit)

> **N.B.** There are smart pointers (handle at runtime by reference
> counting) <https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>

-   **Dynamic memory management (with garbage collectors)**
    -   Haskell \| OCaml \| Go (compile to binaries)
    -   C\# \| Java \| Scala (compile to "portable" bytecode, needs
        custom VM to run like JVM and .NET)

### Dynamic typing (interpreted languages, so dynamic memory management, for scripting mainly)

-   BourneShell \| Python \| Prel \| Ruby \| PHP \| ECMAScript (libs of
    these languages often rely on compiled code)

Lvl 2: What `rustc` do (without lying in the details):
------------------------------------------------------

Rust ⇨ Rust (without macro) ⇨ (module resolution) ⇨ (memory
management) MIR ⇨ ... ⇨ LLVM ⇨ linking (static libs) ⇨
Binary

We can have a taste of Rust without macros with
<https://github.com/mre/cargo-inspect>

We can have a taste of MIR / LLVM / ASM with <https://godbolt.org>

### Inlining example

```rust
{{#include inline.rs}}
```

### Binaries

-   **Executable**
    -   Windows: `.exe` \| Unix (usualy no extension)

> **N.B.** Binary format for Linux is ELF:
> <https://en.wikipedia.org/wiki/Executable_and_Linkable_Format>

-   **Library**
    -   lib static ⇨ compile time (UNIX `.a`, Windows `.lib`)
        language-dependent & platform-indepedent
    -   lib dynamic ⇨ runtime (Linux `.so`, Windows `.dll`, macOS
        `.dylib`) language-independent & platform-dependent

Lvl 3: Foreign Function Interface (FFI)
---------------------------------------

> **N.B.** the Book have a good section on the subject
> <https://doc.rust-lang.org/book/ffi.html>

Today we will try to:

-   Call a Rust function (compiled as a dynamic library) from a Python
    script:
    <https://doc.rust-lang.org/1.5.0/book/rust-inside-other-languages.html>
-   Call a C function (compiled as a static library) from a Rust code:
    <https://rust-embedded.github.io/book/interoperability/c-with-rust.html>

This
<https://stackoverflow.com/questions/43826572/where-should-i-place-a-static-library-so-i-can-link-it-with-a-rust-program>
will help since we have to set up
<https://doc.rust-lang.org/cargo/reference/manifest.html#package-build>
to tell `cargo` where our static lib is!

> **N.B.** <https://doc.rust-lang.org/std/ffi/struct.CStr.html>

**Here is our final code <https://github.com/yvan-sraka/rust-ffi-demo>
:)**

### Lvl 4: Unsafe

![](https://doc.rust-lang.org/nomicon/img/safeandunsafe.svg)

Rust has different rules in unsafe mode, closest to what you know in C,
learn more about in the dedicated guide:
<https://doc.rust-lang.org/nomicon/index.html>

And we try here to reduce `unsafe` bloc to the strict minimum (extern
calls to a C lib).

Cheers, Yvan