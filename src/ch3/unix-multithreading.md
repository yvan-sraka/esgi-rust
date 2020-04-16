# Unix and multithreading

<!-- Hello everyone,

Again, a too-long email, the french version is behind! -->

## Recap from the last session

- Recall previous episode:
    * Rust philosophy "zero-cost abstractions", play a bit with `cargo inspect`, `cargo fmt`, `cargo clippy`

- Let's talk a bit about POSIX, UNIX, Linux, etc ... and file abstraction
    * Named pipe example: `mkfifo` (to send a synchronous message)
    * What about serialization? Do you know protocol buffer? <https://developers.google.com/protocol-buffers>
    (SPOILER: shared memory is better) <https://capnproto.org>

```rust
{{#include wrong-mypipe.rs}}
```

```rust
{{#include mypipe.rs}}
```

- A multi-thread parallel cat?
    * Talk about process scheduling, etc...
    * Show `htop` tree view

```rust
{{#include iterative-cat.rs}}
```

```rust
{{#include parallel-cat-verbose.rs}}

```rust
{{#include parallel-cat.rs}}
```

## To go further

- Learn about File Descriptor <https://en.wikipedia.org/wiki/File_descriptor>
- IOStream Is Hopelessly Broken <https://www.moria.us/articles/iostream-is-hopelessly-broken/>
- Writing an OS in Rust <https://os.phil-opp.com/>
- Why is a Rust executable large? <https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html>
- Smart pointers in Rust <https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>
- Rust sucks because ... <https://wiki.theory.org/index.php/YourLanguageSucks#Rust_sucks_because>
- `unfork` <https://github.com/whitequark/unfork>
- <https://github.com/sebasmagri/rust-concurrency-patterns>

## Prepare the next session

Kkeep talking about Unix tools for binaries analysis:

- <https://lldb.llvm.org>
- <https://godbolt.org>
- <https://en.wikipedia.org/wiki/Executable_and_Linkable_Format>
- <https://en.wikipedia.org/wiki/GNU_Binutils>
- <https://en.wikipedia.org/wiki/Strace>

We will play with FFI: Foreign Function Interface

I wish all of you enjoyed the end-of-year celebrations.

Best, Yvan

P.S. <https://xkcd.com/835/> & <https://xkcd.com/2248/>

<!--

Bonjour tous le monde,

Encore une fois, un e-mail trop long:

## Récapitulatif du dernier cours

- Rappel de l'épisode précédent:
   * La philosophie de Rust "zero-cost abstractions" -> jouons un peu avec `cargo inspect`, `cargo fmt`, `cargo clippy`

- Parlons un peu de POSIX, UNIX, Linux, etc ... et de l'abstraction de fichiers
   * Exemple de pipe nommé: `mkfifo` (pour envoyer un message synchrone)
   * Qu'en est-il de la sérialisation? Connaissez-vous Protocol Buffer? -> <https://developers.google.com/protocol-buffers>
       (SPOILER: la mémoire partagée c'est mieux) -> <https://capnproto.org>

```rust
{{#include wrong-mypipe.rs}}
```

```
```rust
{{#include mypipe.rs}}
```

- Un `cat` parallèle multi-threadé ?
   * Parlons de la planification des processus, etc ...
   * `htop` en mode `tree view`

```rust
{{#include iterative-cat.rs}}
```

```rust
{{#include parallel-cat-verbose.rs}}

```rust
{{#include parallel-cat.rs}}
```

En français, vous pouvez lire ce cours d'OS <https://darnuria.eu/2019-2020_os> pour vous rafraichir la mémoire !

## Pour aller plus loin

- Connaissez vous les descripteurs de fichier <https://en.wikipedia.org/wiki/File_descriptor>
- IOStream est désespérément cassé <https://www.moria.us/articles/iostream-is-hopelessly-broken/>
- Écrire un OS en Rust <https://os.phil-opp.com/>
- Pourquoi un exécutable Rust est-il volumineux ? <https://lifthrasiir.github.io/rustlog/why-is-a-rust-executable-large.html>
- Pointeurs intelligents dans Rust <https://doc.rust-lang.org/book/ch15-00-smart-pointers.html>
- Rust est nul parce que ... <https://wiki.theory.org/index.php/YourLanguageSucks#Rust_sucks_because>
- `unfork` <https://github.com/whitequark/unfork>
- <https://github.com/sebasmagri/rust-concurrency-patterns>

## Pour préparer le prochain cours (parlons d'outils Unix pour l'analyse de binaires)

   * <https://lldb.llvm.org>
   * <https://godbolt.org>
   * <https://en.wikipedia.org/wiki/Executable_and_Linkable_Format>
   * <https://en.wikipedia.org/wiki/GNU_Binutils>
   * <https://en.wikipedia.org/wiki/Strace>

Nous allons jouer avec les FFI: Foreign Function Interface

J'espères que vous avez tous passés de très bonnes fêtes de fin d'années,

Amitiés, Yvan

-->