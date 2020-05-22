Rust syntax
===========

<!-- Hello everyone,

A quick mail about "Systems and networks programming in Rust" lecture, read it entirely, there is some homework for you at the end. -->

Rust basics (Introduction, Syntax, Error Handling)
--------------------------------------------------

-   0.  Schedule, purpose, and rules of the class (planning,
        assignments, grades, etc..)

-   1.  What's systems programming? Why systems (and networks)
        programming? Why Rust?

-   2.  Setup a Rust dev environment with <https://rustup.rs>, and a
        code editor (<https://www.rust-lang.org/tools>) with RLS support
        (<https://github.com/rust-lang/rls>) and be sure not to have
        only the VSCode extension but to install it with the command
        `rustup component add rls rust-analysis rust-src`

> **N.B.** People on Windows should read this
> <https://github.com/rust-lang/rustup.rs/#working-with-rust-on-windows>,
> and installing Visual Studio C/C++ tools suite (like link.exe to link
> program) using Visual Studio Installer.

I also recommend to get you a WLS and install Rust with rustup in
bash.exe, since starting from course 3 we will play with Linux file
abstraction.

-   3.  Get familiar with language syntax by playing a bit with
        <https://github.com/rust-lang/rustlings>

-   4.  Theoretical recap about the semantics of a Rust program (we talk
        about functions VS macros, type inference, enumeration types
        <https://doc.rust-lang.org/std/result/enum.Result.html>)

-   5.  Show off basic tooling: how to create a binary/library with
        cargo, how to add a dependency form crate.io to our project, we
        start writing a little **MASTERMIND** game (Here is a link to
        the full implementation
        <https://github.com/yvan-sraka/mymastermind>) SLIDES are here:
        <https://github.com/yvan-sraka/mymastermind-slides>

Further information could be found in THE RUST BOOK ⇨
<https://doc.rust-lang.org/stable/book/> ⇦ (what we do in class matches
the first 3 chapters of the book)

Alternatively in <https://learning-rust.github.io/>!

Homework (mandatory)
---------------------

<!-- deadline: push it before the class -->

Make a PR (Pull-Request) on this repository that fixes this code:
<https://github.com/yvan-sraka/base64decode/blob/master/src/main.rs>

It will be automatically validated by GitHub Actions (Continuous
Integration), so if it's green you already have a good grade. I will
give you extra points if you succeed to reduce the size of the codebase
without breaking it!

You can test your code by trying to decode this secret message:

    TGEgcmFjbGV0dGUgKEJyYXRjaMOkcywgwqsgZnJvbWFnZSDCuyByw7R0aSwgZW4gc3Vpc3NlIGFsbGVtYW5kKSBlc3QgZCd1bmUgcGFydCB1biBmcm9tYWdlIChsZSBvdSBsYSByYWNsZXR0ZSkgb3JpZ2luYWlyZSBkdSBjYW50b24gZHUgVmFsYWlzIGVuIFN1aXNzZSwgZXQgZCdhdXRyZSBwYXJ0LCB1bmUgcmVjZXR0ZSBkZSBjdWlzaW5lIHRyYWRpdGlvbm5lbGxlIGV0IGVtYmzDqW1hdGlxdWUgZGUgbGEgY3Vpc2luZSBzdWlzc2UsIGNvbm51ZSBkYW5zIGxlIG1vbmRlIGVudGllciwgdmFyaWFudGUgZGVzIGZvbmR1ZXMgYXUgZnJvbWFnZSwgw6AgYmFzZSBkZSBjZSBmcm9tYWdlIGZvbmR1LCByYWNsw6kgYXUgZnVyIGV0IMOgIG1lc3VyZSBxdeKAmWlsIGZvbmQsIGV0IHNlcnZpZSB0cmFkaXRpb25uZWxsZW1lbnQgYXZlYyBkZXMgcG9tbWVzIGRlIHRlcnJlIGVuIHJvYmUgZGVzIGNoYW1wcyBldCBhY2NvbXBhZ27DqWUgZGUgbMOpZ3VtZXMgYXUgdmluYWlncmUgKGNvcm5pY2hvbnMsIG9pZ25vbnMpLg==

⚠️ Reminder, the next class will start with a quick test (don't be
late), it will be graded and take the form of a QCM.

Cheers, Yvan

<!--

Bonjour tout le monde,

Un petit courrier suite au premier cours de "Programmation système et réseau en Rust", lisez-le entièrement, il y a quelques devoirs à faire pour vous à la fin.

## Dans l'épisode précédent

Notions de base sur Rust (Introduction, Syntaxe, Traitement des erreurs)

- 0. Horaire, but et règles de la classe (planning, devoirs maisons, notes, etc.)

- 1. Qu'est-ce que la programmation système ? Pourquoi programmer des systèmes (et des réseaux) ? Pourquoi Rust ?

- 2. Configurez un environnement de développement Rust avec <https://rustup.rs> et un IDE (<https://www.rust-lang.org/tools>) avec support RLS (<https://github.com/rust-lang/rls>) vous avez besoin de l'extension VSCode, mais de lancer cette commande `rustup component add rls rust-analysis rust-src`

> **N.B.** Les utilisateurs de Windows doivent lire ce guide <https://github.com/rust-lang/rustup.rs/#working-with-rust-on-windows> et installer la suite d'outils Visual Studio C / C++ (contenant par exemple link.exe) à l'aide de Visual Studio Installer.

Je vous recommande également de vous procurer un WLS et d'installer Rust avec rustup dans bash.exe, puisqu'à partir du cours 3, nous allons jouer avec l'abstraction de fichiers Linux.

- 3. Familiarisez-vous avec la syntaxe du langage en jouant un peu avec <https://github.com/rust-lang/rustlings>

- 4. Récapitulation théorique de la sémantique d'un programme Rust (on a parlé de fonctions VS macros, d'inférence de types, d'énumérations <https://doc.rust-lang.org/std/result/enum.Result.html>)

- 5. Les outils de base : comment créer une bibliothèque / binaire avec cargo, comment ajouter une dépendance crate.io à notre projet, nous avons commencé à écrire un petit jeu MASTERMIND (voici un lien vers l'implémentation complète <https://github.com/yvan-sraka/mymastermind>)

Des informations complémentaires sont disponibles dans THE RUST BOOK ⇨ <https://doc.rust-lang.org/stable/book/> ⇦ (ce que nous avons fait en classe correspond aux 3 premiers chapitres du livre)

Ou alternativement dans <https://learning-rust.github.io/>!

Des exercices d'introduction aux concepts de bases du langage, en français, sont disponiblent ici <https://framagit.org/darnuria/rust-initiation/> !

## Devoir maison (obligatoire) - date limite: à envoyer avant le cours

Créez une PR (Pull-Request) sur ce repos qui corrige les bugs de ce code: <https://github.com/rust-esgi/base64decode>

Il sera automatiquement validé par GitHub Actions (Intégration Continue), donc si c'est vert, vous avez déjà une bonne note. Je vous donnerai des points supplémentaires si vous parvenez à réduire la taille du code sans le casser!

Vous pouvez tester votre code en essayant de décoder ce message secret:

    TGEgcmFjbGV0dGUgKEJyYXRjaMOkcywgwqsgZnJvbWFnZSDCuyByw7R0aSwgZW4gc3Vpc3NlIGFsbGVtYW5kKSBlc3QgZCd1bmUgcGFydCB1biBmcm9tYWdlIChsZSBvdSBsYSByYWNsZXR0ZSkgb3JpZ2luYWlyZSBkdSBjYW50b24gZHUgVmFsYWlzIGVuIFN1aXNzZSwgZXQgZCdhdXRyZSBwYXJ0LCB1bmUgcmVjZXR0ZSBkZSBjdWlzaW5lIHRyYWRpdGlvbm5lbGxlIGV0IGVtYmzDqW1hdGlxdWUgZGUgbGEgY3Vpc2luZSBzdWlzc2UsIGNvbm51ZSBkYW5zIGxlIG1vbmRlIGVudGllciwgdmFyaWFudGUgZGVzIGZvbmR1ZXMgYXUgZnJvbWFnZSwgw6AgYmFzZSBkZSBjZSBmcm9tYWdlIGZvbmR1LCByYWNsw6kgYXUgZnVyIGV0IMOgIG1lc3VyZSBxdeKAmWlsIGZvbmQsIGV0IHNlcnZpZSB0cmFkaXRpb25uZWxsZW1lbnQgYXZlYyBkZXMgcG9tbWVzIGRlIHRlcnJlIGVuIHJvYmUgZGVzIGNoYW1wcyBldCBhY2NvbXBhZ27DqWUgZGUgbMOpZ3VtZXMgYXUgdmluYWlncmUgKGNvcm5pY2hvbnMsIG9pZ25vbnMpLg ==

⚠️ Rappel, le prochain cours commencera par un test rapide (il ne faut pas arriver en retard), il sera noté et prendra la forme d'un QCM.

Amitiés, Yvan

-->