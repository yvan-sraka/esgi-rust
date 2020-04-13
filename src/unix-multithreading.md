Hello everybody,

Again, a too-long email, the french version is behind!

# ⚠️ IMPORTANT

I intend to assign you grades and I need you to be sure that I have the good matching between your mails, GitHub IDs, and names.

## Recap from the last session

- Recall previous episode:
    * Rust philosophy "zero-cost abstractions" => play a bit with `cargo inspect`, `cargo fmt`, `cargo clippy`

- Let's talk a bit about POSIX, UNIX, Linux, etc ... and file abstraction
    * Named pipe example: `mkfifo` (to send a synchronous message)
    * What about serialization? Do you know protocol buffer? => <https://developers.google.com/protocol-buffers>
        (SPOILER: shared memory is better) => <https://capnproto.org>

- A multi-thread parallel cat?
    * Talk about process scheduling, etc...
    * Show `htop` tree view, play with small tests using Valgrind / GDB

```rust
{{#include iterative-cat.rs}}
```

```rust
{{#include parallel-cat-verbose.rs}}

```rust
{{#include parallel-cat.rs}}
```

e.g. Valgrind will tell you that this code have a big memory leak:

```c
#include "stdlib.h"
#include "unistd.h"

int main(void) {
    while (1) {
        malloc(20);
        sleep(1);
    }
    return 0;
}
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

## Prepare the next session (keep talking about Unix tools for binaries analysis)

- <https://lldb.llvm.org>
- <https://godbolt.org>
- <https://en.wikipedia.org/wiki/Executable_and_Linkable_Format>
- <https://en.wikipedia.org/wiki/GNU_Binutils>
- <https://en.wikipedia.org/wiki/Strace>

We will play with `=>` FFI: Foreign Function Interface

I wish all of you enjoyed end-of-year celebrations.

Best, Yvan

P.S. <https://xkcd.com/835/> & <https://xkcd.com/2248/>

---

Bonjour tous le monde,

Encore une fois, un e-mail trop long:

# ⚠️ IMPORTANT

Pour vous attribuer des notes, j'ai besoin d'avoir la bonne correspondance entre vos e-mails, identifiants GitHub et vos nom / prénom.

## Récapitulatif du dernier cours

- Rappel de l'épisode précédent:
   * La philosophie de Rust "zero-cost abstractions" -> jouons un peu avec `cargo inspect`, `cargo fmt`, `cargo clippy`

- Parlons un peu de POSIX, UNIX, Linux, etc ... et de l'abstraction de fichiers
   * Exemple de pipe nommé: `mkfifo` (pour envoyer un message synchrone)
   * Qu'en est-il de la sérialisation? Connaissez-vous Protocol Buffer? -> <https://developers.google.com/protocol-buffers>
       (SPOILER: la mémoire partagée c'est mieux) -> <https://capnproto.org>

- Un `cat` parallèle multi-threadé ?
   * Parlons de la planification des processus, etc ...
   * `htop` en mode `tree view`, jouons avec de petits tests en utilisant Valgrind / GDB

```rust
{{#include iterative-cat.rs}}
```

```rust
{{#include parallel-cat-verbose.rs}}

```rust
{{#include parallel-cat.rs}}
```

ex: Valgrind va vous dire que ce programme à une grose fuite mémoire :

```c
#include "stdlib.h"
#include "unistd.h"

int main(void) {
    while (1) {
        malloc(20);
        sleep(1);
    }
    return 0;
}
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

Nous allons jouer avec les `=>` FFI: Foreign Function Interface

J'espères que vous avez tous passés de très bonnes fêtes de fin d'années,

Amitiés, Yvan