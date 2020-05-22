Introduction
============

What's that? Another Rust book?
-------------------------------

I want to learn Rust, and what I want to do with is some system and
network programming (big topic indeed):

-   There is already a really good Rust course online, like
    <http://intorust.com/>, and of course, the official
    [Book](https://doc.rust-lang.org/book/) or [Programming
    Rust](http://shop.oreilly.com/product/0636920040385.do) that will be
    more suitable for you if you come from a C++ background.
-   There is no better way to learn system programming than reading
    syscalls man pages and try to implement things (look at all the
    project assignments in this book)!

But this could be a bit harsh, the purpose of the course is to organize
the knowledge in increasing difficulty. I try to give you a gentle
introduction to many general computer system concepts more than laying
in implementation details.

> ⚠️ This book comes from notes I take from the lectures I give in an
> engineering school. This is a **Work In Progress**: some
> chapters are made of just a bunch of links, some contain code from live coding I
> gave in class.
>
> I will try to progressively improve the content of it to a more
> readable and agnostic form, intended to a public that has no previous
> knowledge in Rust and only basis in computer systems, but that already
> knows at least one "low level" language, I'm thinking about C.

Knowledge in Linux systems is not required but would really help the
interesting of the topics introduces after, even if I give a trick to
get Rust on non-Unix machine, some parts of the courses are heavily
Linux-oriented.

Most of the course is organized around code examples that I invite you to
try on your personal computer.

I have myself a bias since I'm really into the programming language and
compilation field, so we often take few detours to explain how things works
under the hood, even if it's related to Rust mechanisms more than
system programming itself.

Program of the next lectures
----------------------------

-   Rust basics (goals of the language and lectures, syntax and features
    overview)
-   Rust advanced (static automated memory management: Ownership &
    Borrowing)
-   Files (standard IO, synchronous message through pipes), Tasks
    (process, thread) and other POSIX stuff
-   FFI, unsafe world (let's talk about "lifetimes") and the C runtime
    (stack & heap)
-   Sockets & HTTP (finally, something fun for the last class)
-   WebAssembly (why not ... maybe it will become a thing
    someday)
-   Fast, safe & beyond (in place of a conclusion, general guidelines to
    improve our softwares)

Syscalls that every programmer should know
------------------------------------------

Disclaimer: we don't have enough time in class to talk about all
interesting syscalls, so I take the chance to give you here pills to go
further by yourselves, the ultimate goals of this lecture is to show you
everything about:

-   `errno`, filesystem API (`fstat`,`lstat`, `open`,`close`,
    `stat`,`read`, `write`,`mmap`), networking API (`poll`)
-   memory API (`mmap`,`mprotect`, `munmap`)
-   signal API (`rt_sigaction`,`rt_sigprocmask`, `rt_sigtreturn`)
-   `dup`,`dup2` which are important (also `madvise` but it's a bit less
    essential)
-   IPC (InterProcess Communication) / shm (shared memory) are something
    you can't miss, so check `shmget`,`shmat`, `shmctl`
-   `sendfile` (this is the 64-bit syscall to know when doing
    high-performance networked system)

I would explain the costs of context switching kernel/user land, what
happens when you file a userland buffer to the kernel if you want to be
edgy, `copy_to_user` on the kernel side.

I would have written a syscall, to show how it's simple!

-   `fork`,`execve` also important, explaining `execve` is not easy, and
    we even use it in Python
-   `flock` semaphores,`fsync` in filesystem if there is time (`flock`
    is very "interesting")

I would be left to extend the filesystem API (`fdatasync`, `truncate`,
`ftruncate`, `getdents`, `getcwd`, `chdir`, `fchdir`, `rename`, `mkdir`,
`rmdir`, `create`, `link`, `unlink`, `symlink`, `readlink`, `chmod`,
`fchmod`, `fchown`, `chown`, `lchown`, `umask`)

I would have left that and I would have embarked on `ptrace`, the
privileges linux side, `getgid`, `setuid`, `setgid`, `setsid`,
`getppid`, `getpgid`, `getpgrp`, `setresuid`, `getresuid`, and so on ...

If affinities, I would have spoken quickly of capabilities,
`capget`,`capset`

If I still have some time left, I would have talked about an interesting
system API, `chroot`,`pivot_root`, `ioctl`,`mount` in particular :)

If I was on the filesystem API or not, I would have talked about
eXtended attributes, so `xattrs` syscalls!

If I'm on networking and we have done a lot of things, we can discuss
`io_XXX` with AIO or `epoll_YYY` and that would allow me to talk about
BSD

And frankly, ultimately, pick your poison:

-   `inotify` API (it should go into the course on the filesystem API in
    real life, but hey, it requires a little maturity)
-   `keyctl` API
-   `kexec` API
-   `splice`, `vmsplice`
-   `perf_event_open`
-   System Tap

And for the bests, there are also the `sched` & `cgroups` ...

So a big program, that right know this lecture does not cover at all,
but maybe at some point, it will :)

Greetings
---------

-   Axel Viala, which give the same lecture to another set of classes,
    in his way, but we highly collaborate in the project design
-   Ryan Lahfa, that give me really good guidelines on how to improve
    these lectures and general feedback about the content presented here

Cheers, Yvan

<!--

## Programme des prochaines séances

- Rust avancés (gestion automatique de la mémoire statique: « ownership » et « borrowing »)
- Fichiers (E / S standard, message synchrone via des pipes), tâches (processus, thread) et autres POSIXeries
- FFI,  unsafe mode (parlons de « lifetimes ») et binaire
- *** NETWORK (enfin, quelque chose d'amusant pour le dernier cours) ***

-->