Hello everyone!

You will find a French version of this mail bellow, read it carefully and enjoy a homework bit trickier than the previous time:


# Recap from the previous session

- QCM is a huge success, the mean of the class is X / 20 (negative points are on me this time)
- Correction of last homework: <https://gist.github.com/yvan-sraka/94638a5dd95f46cdaecf5ab4d7ed2676>
- I strongly advise you to try to finish rustlings to be more comfortable with basic language features <https://github.com/rust-lang/rustlings>
- The small Rust code I wrote in class to play with ownership and borrowing:

```rust
{{#include borrowing-example-strings.rs}}
```

**REMINDER**: Rule of thumbs of Rust -> they cannot have both aliasing AND mutability!

```rust
{{#include borrowing-example-colors.rs}}
```

# Mandatory for the next session

⚠️ Have a UNIX system with Rust installed inside:

For those that run a Windows machine, I highly recommend the installation of a WSL (Windows Subsystem for Linux) <https://docs.microsoft.com/en-us/windows/wsl/install-win10>

and the nice VSCode extension <https://code.visualstudio.com/remote-tutorials/wsl/run-in-wsl> that allows you to run `code` command remotely in bash.exe shell!

(and, of course, to have a working Rust dev environment with RLS setup <https://github.com/rust-lang/rls>)


# Memory 


# Go deeper into Rust

We, at this point cover, all of 6 first chapters, and most of 7, 8 and 9 of the Rust Book <https://doc.rust-lang.org/stable/book/>

We will not advance to much in Rust specific feature after this point (I will not make a class about trait e.g.), I let you free of learning more about it or not!

I give you here three handy tools that will help you with homework and graded project:
- <https://github.com/rust-lang/rust-clippy>
- <https://github.com/rust-lang/rustfmt>
- <https://github.com/mre/cargo-inspect> (play with it!)


# Go deeper into memory

Try to create a small C program that creates a memory leak (like a loop that malloc but never free) and open it in Valgrind, translates the program in Rust and redo the test.

Do you know that the program stack has a fixed space size in memory? What's happen when you fill it all with function calls? -> a stack overflow!

You can look at this minimalist malloc implementation from mine, using mmap syscall (read its man) to allocate memory pages: <https://github.com/yvan-sraka/malloc>

Supplementary links to feed your curiosity (bonus, not mandatory):

- There no null pointers in Rust! Why? Watch "Null References: The Billion Dollar Mistake" from Tony Hoare <https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/>
- Some more readings for the brave: "What Every Programmer Should Know About Memory" by Ulrich Drepper from Red Hat <https://people.freebsd.org/~lstewart/articles/cpumemory.pdf>
- [Fear not the Rust Borrow Checker](http://www.squidarth.com/rc/rust/2018/05/31/rust-borrowing-and-ownership.html)


# Homework due to next session

You have to recode a small pipe-like program, working like this:

```
$ mypipe --in fortune --out cowsay
```

```
 _______________________________________
/ Q: What's tiny and yellow and very,   \
| very, dangerous? A: A canary with the |
\ super-user password.                  /
 ---------------------------------------
          \   ^__^
           \  (oo)\_______
              (__)\       )\/\
                  ||----w |
                  ||     ||
```

You can use <https://clap.rs> to parse the command-line arguments, and also follow the guide <https://rust-lang-nursery.github.io/cli-wg/>

Upload your code by doing a PR here: <https://github.com/yvan-sraka/mypipe>


# Big Project

I will present during the next class the final project on which you will be evaluated. You're free to come with your idea of an alternative project if you have already in mind something that you want to code in Rust. I will accept any idea that could be reasonably doable by a group of 3 or 4 students (chosen randomly), that implies features specific to systems or networks programming (think about playing with binary encoding, intense computing with concurrent programming, low-level binding with another library or just any funny syscalls, etc…)!

Cheers, Yvan

---

Bonjour à tous!

L'anglais ce n'est pas votre truc, je ne vous en veux pas :)

# Récapitulatif de la session précédente

- Le QCM est un énorme succès, la moyenne de la classe est de X / 20 (les points négatifs sont pour moi cette fois)
- Correction du dernier devoir maison : <https://gist.github.com/yvan-sraka/94638a5dd95f46cdaecf5ab4d7ed2676>
- Je vous conseille vivement d'essayer de finir les exercices « rustlings » pour être plus à l'aise avec les fonctionnalités de base du langage <https://github.com/rust-lang/rustlings>
- Le petit code Rust que j'ai écrit en classe pour jouer avec les concepts d'« ownership » et de « borrowing » :

```rust
{{#include borrowing-example-strings.rs}}
```

**RAPPEL:** règle d'or de Rust -> il ne peut pas avoir à la fois de l'aliasing ET de la mutabilité!

```rust
{{#include borrowing-example-colors.rs}}
```

> **N.B.** <https://blog.guillaume-gomez.fr/Rust> donne des bonnes explications (en français) du modèle mémoire de Rust !

# Obligatoire pour la prochaine session

⚠️ Avoir un système UNIX avec Rust installé dessus :

Pour ceux qui exécutent une machine Windows, je recommande vivement l'installation d'un WSL (Sous-système Windows pour Linux) <https://docs.microsoft.com/en-us/windows/wsl/install-win10>

et de l'extension VSCode qui va bien <https://code.visualstudio.com/remote-tutorials/wsl/run-in-wsl> qui vous permet d'exécuter la commande `code` à distance dans un shell bash.exe!

(et, bien sûr, d'avoir un environnement de développement Rust fonctionnel avec RLS activé <https://github.com/rust-lang/rls>)


# Aller plus loin dans Rust

Nous avons couvert jusqu'à présent les 6 premiers chapitres et la plupart des 7, 8 et 9 du Rust Book <https://doc.rust-lang.org/stable/book/>

Nous n'avancerons pas beaucoup dans les fonctionnalités spécifiques de Rust à partir de maintenant (je ne ferai, par exemple, pas de cours sur les traits), je vous laisse libre d'en apprendre davantage sur le sujet ou pas!

Je vous donne ici trois outils pratiques qui vous aideront avec vos devoirs et votre projet noté:
- <https://github.com/rust-lang/rust-clippy>
- <https://github.com/rust-lang/rustfmt>
- <https://github.com/mre/cargo-inspect> (jouez avec !)


# Aller plus loin dans la mémoire

Essayez de créer un petit programme en C qui crée une fuite de mémoire (comme une boucle qui « malloc » mais qui ne « free » jamais) et ouvrez-le dans Valgrind, traduisez le programme en Rust et refaites le test.

Savez-vous que la pile d'un programme a une taille fixe dans l'espace mémoire ? Que se passe-t-il lorsque vous dépassez l'espace disponible avec trop d'appels de fonction ? -> « stack overflow » !

Vous pouvez regarder cette implémentation malloc minimaliste, basé sur l'appel système mmap (lisez son manuel) pour allouer des pages de mémoire : <https://github.com/yvan-sraka/malloc>

Quelques liens supplémentaires pour nourrir votre curiosité (en bonus, non obligatoire):

- Il n'y a pas de pointeurs « null » dans Rust! Pourquoi? Regardez "Null References: The Billion Dollar Mistake" de Tony Hoare <https://www.infoq.com/presentations/Null-References-The-Billion-Dollar-Mistake-Tony-Hoare/>
- Quelques lectures supplémentaires pour les plus courageux: "What Every Programmer Should Know About Memory" de Ulrich Drepper chez Red Hat <https://people.freebsd.org/~lstewart/articles/cpumemory.pdf>
- [Fear not the Rust Borrow Checker](http://www.squidarth.com/rc/rust/2018/05/31/rust-borrowing-and-ownership.html)


# Devoirs maison pour la prochaine session

Vous devez recoder un petit programme qui fonctionne comme pipe | et s'appelle comme ceci:

```
$ mypipe --in fortune --out cowsay
```

```
 _______________________________________
/ Q: What's tiny and yellow and very,   \
| very, dangerous? A: A canary with the |
\ super-user password.                  /
 ---------------------------------------
          \   ^__^
           \  (oo)\_______
              (__)\       )\/\
                  ||----w |
                  ||     ||
```

Vous pouvez utiliser <https://clap.rs> pour parser les arguments de la ligne de commande, également vous aider du guide <https://rust-lang-nursery.github.io/cli-wg/>

Soumettez votre code en faisant une PR ici: <https://github.com/yvan-sraka/mypipe>


# Projet final

Lors du prochain cours, je présenterai le projet final sur lequel vous serez évalué. Vous êtes libre de proposer des idées de projets alternatifs si vous avez déjà en tête quelque chose que vous souhaitez coder dans Rust. J'accepterai toute idée raisonnablement réalisable par un groupe de 3 ou 4 étudiants (choisis au hasard), qui implique des fonctionnalités spécifiques à la programmation système ou réseau (pensez à jouer avec un encodage binaire, des programmes concurrents qui font des calculs, de l'interopérabilité bas niveau avec une autre bibliothèque ou des appels systèmes, etc ...)!

Amitiés, Yvan
