`rsh`: a shell for the future written in Rust
=============================================

> This project was nicely brought by Axel and it's a nice opportunity to
> maybe reuse what you did in the `mypipe` assignment!

(Reminders) Process
-------------------

During this practical workshop, we will start by demystifying process
abstraction by making a mini command prompt very simple.

Whether you are on Linux, macOS or Windows your programs are isolated
each other, this abstraction is called the process, sometimes called
"task". This basic mechanism makes it possible to implement stronger
insulations like those used in docker or type virtual machines
[QEMU](https://www.qemu.org/).

But in everyday life the processes allow you to write programs without
having access to other programs on the computer.

> **N.B.**: We will see a close concept seen earlier on last semester
> the concept of
> [*thread*](https://en.wikipedia.org/wiki/Thread_(computing)) or thread
> calculation. A thread is an additional thread in a process! For have
> another program you have to do a process if you want just do
> calculations on another processor a thread is what you need!

A process has its own address space divided into segments, i.e. its own
**stack**, its own **heap**, its own code in the segment **.text**, its
own data known to the compilation **.data** and others with their own
use like space dynamic libraries.

The size of this space depends on your system, on a 32bit system this
space goes from the address `0x0000_0000` to`0xFFFF_FFFF` on 64 bits
from `0x0000_0000_0000_0000` to `0xFFFF_FFFF_FFFF_FFFF` [^1] the formula
for knowing the space maximum addressing is: `pow(2, n) - 1`.

This space is divided as we have seen before into segments, it is your
operating system that orchestrates this abstraction, compilers linkers
**linkers** and assemblers respect it.

Practical micro-shell
=====================

To appropriate the concepts, we will write a program that reproduces the
behavior of a shell. A shell is a program that expects commands and
organizes the execution of programs with a dedicated language in your
favorite terminal manager. There are many, for example, Bash, Zsh, Ksh,
PowerShell (Windows).

Your program at the end of the TP can do at least the actions following:

-   You display a character to invite the user to write
-   You are waiting for an entry on the `STDIN`
-   Try to execute the command
-   Read the status of the command
-   Repeat at the initial stage

We will start with the following hand:

```rust
{{#include 001.rs}}
```

Deployment of the project and inputs/outputs
--------------------------------------------

Create a binary project with cargo:

```shell
cargo new --new micro-shell
    Created binary (application) `micro-shell`.
```

How to compile and then run your program? Run the tests? Where are put
the binaries (in debug mode)?

-   `cargo build`
-   `cargo run`
-   `cargo test`
-   from your current project directory `./target/debug`

Display a character inviting to type a command.

In our micro-shell, we will write a character which indicates that we
can take a command! It's called *prompt* in many languages.

To start, write a `main` function like here. Challenge of the TP do not
use `unwrap` but `expect` to manage errors correctly.

In this main function, we will manipulate the standard `stdout` output
to display a character prompting for input, for example,`>`, you
discover that to write on the `stdout` and that it appears in common
sense will have to be explicit because by default the order of reading
and writing on the `stdin` and `stdout` is not the one that you write in
your program but depending on the availability of the system [^2].

To succeed here are links to the documentation:
[`std::io::stdout`](https://doc.rust-lang.org/std/io/struct.Stdout.htmldoc)
the `flush` method may be useful for you it is implemented by the line
`Write` [^3]. For worries with command display of messages on standard
STDIN and STDOUT streams `Write::flush` is there to force the effective
write/read.

At this point you should have something like this:

```shell
cargo run
>
```

```rust
{{#include 002.rs}}
```

Execution of a Process
======================

The next step is to be able to execute a command typed by a user, in
fact this will require: creating a process and make it execute a program
of our choice, for example, `ls`.

For example, if you write `sl` instead of `ls` [^4].

An error will occur, or if you are in the wrong directory, or have the
wrong rights, short run is full of risks.

To do so, we will use the `std::process` module in the standard library
[doc](https://doc.rust-lang.org/stable/std/process/index.html), this
abstraction offers a portable way between several OS to manage processes
[^5]

Sometimes we need libraries closer to systems like that
[nix](https://github.com/nix-rust/nix) for
[windows](https://lib.rs/os/windows-apis)

A process can execute a process, it gives a parent and a child often
performing and having a child are two separate actions for an OS and run
a new program comes back to create a child then execute. Your children's
program will have its own address space, but it will inherit your open
file descriptors and you will have the responsibility of verifying that
he has finished [^6]

For example, under Linux, the only program without parents is the
process 1 often called `init`. His responsibility is quite important,
all the other processes are children or grandchildren of that one.

Execute a command
-----------------

Successfully execute a command with `std::process::Command::status`.

View command status, why Rust is forcing you to recover the status?

What does your program do while the child is running?

Now manage but for a command with several arguments!

Pipe my programs
================

As you must have seen from our programs, in addition to memory, system
calls and files have access to 3 magic files, inputs `STDIN`,
outputs`STDOUT` and standard error `STDERR`. It allows programs to
communicate with each other and with the user in the context of a
terminal.

We want to be able to communicate very basically two programs between
them, as if you could connect a tube to redirect the exit to the entry
of another program.

We wish we could do something like:

```shell
ls | grep hello
```

Redirects
---------

Writing a basic version or a command without arguments processes a
command without arguments, for example, a simple `ls` redirected in the
`echo` program.

Write a more advanced version where our two programs run really at the
same time (no cheating with `std::process::Command::output`) and handle
multiple arguments to both commands.

Competitive executions: Manage background commands
==================================================

We would now like to manage commands in the background.

Now if a command is issued like this with the series of `>` characters
in front of the command then your shell will not hang.

```shell
> ls &
[1] 21520
latex micro-shell exercises
[1] + Done ls
```

The `21520` corresponds to the process identifier *process id* of the
program that you just launched and the `[1]` to the current command
number internal shell.

What's a process id? Write an implementation basic that manages only one
job in a background task. Write a structure [^7] to store running
programs for your shell. To write a job command which displays the
programs running in the background and their condition.

Program environment
===================

In addition to having a stack, a heap, a segment for the code and
arguments our programs have a little space called environment which
contains lots of information like the `PATH` variable which contains the
paths where to find the executables.

Write a small program (other binary than the shell) `filterEnv` which
executes a command with its arguments as a parameter but filters its
environment to contain only the variable `PATH`, `TERM`, `LANG` and `TZ`

To help you importants
[documentation](https://doc.rust-lang.org/stable/std/process/struct.Command.html#method.envs)
the `std::process::Command::env` function can be useful!

Go further
==========

Here start the bonuses! It's much harder but free to try!

In the exercises, we used the terminal in "managed" mode. say that the
shell has an already existing configuration. You have probably noticed
you can't rewrite what you write and you can't control the display in
the terminal. The solution is to change the terminal mode, for that two
libraries can help you: [`termion`](https://github.com/redox-os/termion)
and [`termios`](https://dcuddeback.github.io/termios-rs/termios/)

You may have noticed that `Ctrl-C` or`Ctrl-Z` etc does not work as
expected? On a Unix system, if you want to redefine this stuff you will
have to discover the signals! A voucher starting point is this piece of
code from [the interfaces section in book of Command Line Applications
in Rust](https://rust-cli.github.io/book/in-depth/signals.html).

So far we have implemented everything using `std::process`. It is a
high-level library, present try writing the code to execute a process
using the crate [`libc`](https://crates.io/crates/libc) which offers
bindings on system libraries in C. The C functions wrapped in `fork` and
`execve` should suit you. Try to write a safe wrapper.

<!--

**Questions - rendu**: Les questions de code et de compréhension sont à
faire, celles notées *bonus* sont optionnelles mais recommandées ! Pour
le rendu un dépôt git sans historique réécrit ni triche sera bien, les
questions doivent être rendues au format *markdown* dans le fichier
`readme.md` à la racine du dépôt. **Taille des groupes :** maximum 2
pensez a noter les noms de votre binôme, votre dépôt contiendra tous les
travaux pratiques.

Introduction
============

Bienvenue dans la suite du cours de programmation systèmes et réseaux.
Au travers ce cours, nous allons visiter des concepts liés aux capacités
qu'offrent votre hardware et vos systèmes d'exploitations, afin de vous
dévoiler la magie qui est cachée par les langages de haut niveau.

L'étendue des concepts est très vaste, nous ne pourront pas tout voir
malheureusement.

Processus
---------

Au cours de ce travail pratique nous allons commencer par démystifier
l'abstraction des processus en réalisant un mini invité de commande très
simple.

Que vous soyez sur Linux, MacOS ou Windows vos programmes sont isolés
les un des autres, cette abstraction s'appelle le processus, parfois
appelée «tâche». Ce mécanisme de base permet d'implémenter des
isolations plus fortes comme celles utilisées dans docker ou les
machines virtuelles type [QEMU](https://www.qemu.org/).

Mais dans la vie de tout les jours les processus permettent d'écrire des
programmes sans avoir accès aux autres programmes sur l'ordinateur.

Attention: Nous reverrons plus tard un concept proche déjà vu au premier
semestre, le concept de
[*thread*](https://en.wikipedia.org/wiki/Thread_(computing)) ou fil de
calcul. Un thread est un fil de calcul en plus dans un processus! Pour
avoir un autre programme il faut faire un processus, si vous voulez
juste faire des calculs sur un autre processeur, un thread est ce qu'il
vous faut!

Un processus possède son propre espace d'adressage découpé en segments,
c'est à dire sa propre pile **stack**, son propre tas **heap**, son
propre code dans le segment **.text**, ses propres données connues à la
compilation **.data** et d'autres avec leur propre usage comme l'espace
des bibliothèques dynamiques.

La taille de cet espace dépend de votre système, sur un système 32bit
cet espace va de l'adresse `0x0000_0000` à `0xFFFF_FFFF`
sur 64 bits de `0x0000_0000_0000_0000` à
`0xFFFF_FFFF_FFFF_FFFF`[^1] la formule pour connaître l'espace
maximum d'adressage est la suivante: $2^n -1$.

Cet espace est découpé comme nous l'avons vu avant en segments, c'est
votre système d'exploitation qui orchestre cette abstraction, les
compilateurs lieurs **linkers** et assembleurs la respectent.

Questions: Rappels de Rust, généralités
---------------------------------------

En Rust à quoi servent les références?

À partager des données sans les copier, ni les déplacer.

Citez en Rust les grandes façons de déclarer ses propres types. Rust est
compilé nativement (assembleur sous forme de code machine) ou compte sur
une machine virtuelle pour s'exécuter?

Rust est compilé nativement.

Imaginons qu'on a un système avec un processeur 8bits, quelle est la
valeur maximale adressable ? Écrire la solution en notation hexadécimale
et décimale.

$2^8 - 1 = 255 = 0xFF$

Donnez votre définition d'un processus citez vos sources!

Pratique - micro-shell
======================

Pour s'approprier les concepts, on va écrire un programme qui reproduit
le comportement d'un shell. Un shell est le programme qui attend des
commandes et organise l'exécution de programmes avec un langage dédié
dans votre gestionnaire de terminal favori. Il en existe de nombreux,
par exemple: Bash, Zsh, Ksh, PowerShell (Windows).

Votre programme à l'issue du TP pourra faire au moins les actions
suivantes:

-   Vous affichez un caractère pour inviter l'utilisateur à écrire
-   Vous attendez une saisie sur la `STDIN`
-   Tentez d'exécuter la commande
-   Relevez le statut de la commande
-   Recommencez à l'étape initiale

On commencera avec le main suivant:

```rust
{{#include 001.rs}}
```

Questions: Deployement du projet et entrées sorties
---------------------------------------------------

Créer un projet binaire avec cargo:

```shell
cargo new --new micro-shell
    Created binary (application) `micro-shell` packageedinline{shell}{}.
```

Comment compiler puis exécuter son programme? Exécuter les test? Où sont
rangés les binaires (en mode debug)?

-   `cargo build`
-   `cargo run`
-   `cargo test`
-   depuis votre répertoire courant du projet `./target/debug`

Afficher un caractère invitant à taper une commande

Dans notre micro-shell on va écrire un caractère qui indique qu'on peut
saisir une commande! Ça s'appelle le *prompt* dans beaucoup de langages.

Pour commencer, écrire une fonction `main` comme ici. Challenge
du TP n'utilisez pas `unwrap` `expect` afin de gérer
correctement les erreurs.

Dans cette fonction main, on va manipuler la sortie standard `stdout`
pour afficher un caractère invitant à la saisie par exemple `>`, vous
allez découvrir que pour écrire sur la `stdout` et que ça s'affiche dans
le bon sens il va falloir être explicite car par défaut l'ordre de
lecture et d'écriture sur la `stdin` et textttstdout n'est pas celui que
vous écrivez dans votre programme mais en fonction de la disponibilité
du système [^2].

Pour réussir voici des liens vers la documentation:
[std::io:stdout](https://doc.rust-lang.org/std/io/struct.Stdout.htmldoc)
la méthode `flush` risque de vous être utile elle est implémentée
par le trait `Write`[^3]. Pour les soucis avec l'ordre
d'affichage des messages sur les flux standards STDIN et STDOUT
`Write::flush` est là pour forcer l'écriture/lecture effective.

A ce stade vous devriez avoir quelque chose comme ça

```shell
cargo run
>
```

```rust
{{#include 002.rs}}
```

Execution d'un Processus
========================

La prochaine étape est de pouvoir exécuter une commande tapée par un
utilisateur, dans les fait cela va nécessiter de: créer un processus et
lui faire exécuter un programme de notre choix, par exemple `ls`.

Par exemple si vous écrivez `sl` au lieu de `ls`[^4].

Une erreur va se produire, ou alors si vous êtes dans le mauvais
répertoire, ou avez les mauvais droits, bref exécuter est plein de
risques.

Pour se faire on va utiliser le module `std::process` de la
bibliothèque standard
[doc](https://doc.rust-lang.org/stable/std/process/index.html), cette
abstraction propose une façon portable entre plusieurs OS pour gérer des
processus[^5]

Parfois on a besoin de bibliothèques plus proches des systèmes telles
que [nix](https://github.com/nix-rust/nix) pour
[windows](https://lib.rs/os/windows-apis)

Un processus peut exécuter un processus, ça donne un parent et un enfant
souvent exécuter et avoir un enfant sont deux actions séparées pour un
OS et exécuter un nouveau programme reviens à créer un enfant puis
exécuter. Votre programme enfant va avoir son propre espace d'adressage,
mais il va hériter de vos descripteurs de fichier ouvert et vous aurez
la responsabilité de vérifier qu'il ai bien terminé[^6]

Par exemple sous Linux le seul programme sans parents c'est le processus
1 souvent appelé `init`. Sa responsabilité est assez importante, tous
les autres processus sont des enfants ou petits enfants de celui-là.

Questions: Executer une commande
--------------------------------

Réussir à exécuter une commande avec
`std::process::Command::status`.

Afficher le statut d'une commande, pourquoi Rust vous force à récupérer
le statut ?

Que fait votre programme pendant que son enfant s'exécute?

Maintenant gérer mais pour une commande avec plusieurs arguments !

Redirections - pipe my programs'
================================

Comme vous avez dû le constater nos programmes, en plus de la mémoire,
des appels systèmes et des fichiers ont accès à 3 fichiers magiques, les
entrées `STDIN`, sorties `STDOUT` et erreur standards `STDERR`. Ça
permet de faire communiquer les programmes entre eux et avec
l'utilisateur dans le contexte d'un terminal.

On souhaite pouvoir faire communiquer très basiquement deux programmes
entre eux, comme si vous pouviez connecter un tube pour rediriger la
sortie dans l'entrée d'un autre programme.

On souhaiterait pouvoir faire quelque chose comme :

```shell
ls | grep hello
```

Questions: Redirections
-----------------------

Donnez avec vos mot une définition d'un tupe entre deux programmes citez
vos sources.[^7]

Écrire une version basique ou une commande sans argument traite une
commande sans arguments par exemple un simple `ls` redirigé dans le
programme `echo`.

Écrire une version plus avancée où nos deux programmes s'exécutent
vraiment en même temps (pas de triche avec
`std::process::Command::output`) et gèrent plusieurs arguments
aux deux commandes.

Executions en concurence: Gérer des commandes en fond
=====================================================

On aimerait maintenant gérer des commandes en tâche de fond.

Maintenant si une commande est lancée de la sorte avec la série de
caractères `>` devant la commande alors votre shell ne se bloquera pas.

```shell
> ls &
[1] 21520
exercices  latex  micro-shell
[1]+  Done ls
```

Le `21520` correspond au processus identifier *process id* du programme
que vous venez de lancer et le `[1]` au numéro de commande en cours
interne du shell.

Questions
---------

C'est quoi un processus id? Citez vos sources. Écrire une implémentation
basique qui gère un seul job en tâche de fond. Écrire une structure [^8]
pour stocker les programmes en cours d'exécution de votre shell. Écrire
une commande jobs qui affiche les programmes en cours d'exécution en
fond et leur état.

Environnement de programmes
===========================

En plus d'avoir une stack, une heap, un segment pour le code et les
arguments nos programmes ont un petit espace appelé environnement qui
contient plein d'informations comme la variable `PATH` qui contient les
chemins où trouver les exécutables.

questions
---------

Écrire un petit programme (autre binaire que le shell) `filterEnv` qui
exécute une commande avec ses arguments en paramètre mais filtre son
environnement pour contenir seulement la variable `PATH` `TERM` `LANG`
et `TZ`

Pour vous aider la
[https://doc.rust-lang.org/stable/std/process/struct.Command.html\#method.envs](documentation)
de la fonction `std::process::Command::env` peut vous être
utile !

Aller plus loin
===============

Ici commencent les bonus ! C'est beaucoup plus dur attention mais libre
à vour de tenter!

Dans les exercices on a utilisé le terminal en mode «managé», c'est à
dire que le shell a une configuration déjà existante. Vous avez
probablement remarqué vous ne pouvez pas réécrire ce que vous écrivez et
vous ne pouvez pas vraiment contrôler l'affichage dans le terminal. La
solution est de changer le mode du terminal, pour cela deux
bibliothèques peuvent vous aider:
[https://github.com/redox-os/termion](termion) et
[https://dcuddeback.github.io/termios-rs/termios/](termios)

Vous avez peut être remarqué que `Ctrl-C` ou `Ctrl-Z` etc ne
fonctionnent pas comme attendu? Sous un système Unix, si vous voulez
redéfinir ce genre de choses vous devrez découvrir les signaux! Un bon
point de départ est ce bout de code issu du livre sur les interfaces en
ligne de commande
[https://rust-cli.github.io/book/in-depth/signals.html](cli app book).

Jusqu'à présent nous avons tout implémenté a l'aide de
`std::process`. Il s'agit d'une bibliothèque de haut niveau, à
présent essayez d'écrire le code pour exécuter un processus en utilisant
la crate [https://crates.io/crates/libc](libc) qui propose des bindings
sur les bibliothèques systèmes en C. Les fonctions C wrappé `fork` et
`execve` devraient vous convenir. Tentez d'écrire un wrapper safe.

[^1]: En décimal : $2^{64} - 1 = 18446744073709551615$ C'est très
    grand ...

[^2]: si ça vous intéresse voici un bon article:
    [https://medium.com/\@JoeKreydt/stdout-more-like-stdout-of-order-rust-1f9acc016e89](Stdout? More like Stdout of Order! (Rust) par Joe Kreydt)

[^3]: Les Traits comme `Write` sont un système assez proche des
    interfaces que vous avez sûrement ailleurs, ce sont des contrats qui
    peuvent être rempli par des types

[^4]: Pour la blague : Il existe un programme satirique `sl` *steam
    locomotive*.

[^5]: Attention, c'est une abstraction, chaque système a ses
    particularités par exemple sous Linux et MacOS il y a `fork` et
    `execve` pour créer et exécuter un processus.

[^6]: Sinon ça fait un [processus
    zombie](https://en.wikipedia.org/wiki/Zombie_process) et personne
    n'aime les zombies, si vous voulez une image pour illustrer [Zombie
    processes by Daniel
    Stori](https://turnoff.us/geek/zombie-processes/)

[^7]: La page wikipédia
    [pipeline](https://en.wikipedia.org/wiki/Pipeline_%28Unix%29) peut
    vous aider.

[^8]: Vous pouvez utilisé une `std::collections::VecDeque` ou un
    `Vec` dans votre réalisation

-->

[^1]: In decimal: `pow(2, 64) - 1 = 18446744073709551615` This is very
    big ...

[^2]: if you are interested here is a good article: [Stdout? More like
    Stdout of
    Order!](https://medium.com/@JoeKreydt/stdout-more-like-stdout-of-order-rust-1f9acc016e89)

[^3]: Traits like `Write` are fairly close to interfaces that you surely
    have elsewhere, these are contracts that can be filled by types

[^4]: For the joke: There is a satirical program `sl` *steam
    locomotive*.

[^5]: Be careful, it's an abstraction, each system has its own
    particularities for example under Linux and MacOS there is `fork`
    and `execve` to create and execute a process.

[^6]: Otherwise it makes a [process
    zombie](https://en.wikipedia.org/wiki/Zombie_process) and no one
    don't like zombies, if you want a picture to illustrate [Zombie
    processes by Daniel
    Stori](https://turnoff.us/geek/zombie-processes/)

[^7]: You can use a `std::collections::VecDeque` or a `Vec` in your
    achievement
