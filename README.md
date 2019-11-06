# esgi-rust

Cours de système et réseaux de Rust.

## Journée 0 - Bases de Rust

Plongée dans le langage Rust:

### Planning

- 09:45 - 10:00 Présentation générales
- 10:00 - 10:20 Installation de rust avec rustup
- 10:20 - 11:15 Exercices sur rustlings bases
- 11:15 - 11:30 Pause
- 11:30 - 13:00 Exercices sur rustlings jusque aux struct

- 13:00 - 14:00 Pause manger

- 14:00 - 14:20 Fin exercice sur rustlings
- 14:20 - 15:30 application sur les structures `struct Point2D`
- 15:30 - 15:45 Pause
- 15:45 - 16:30 Recapitulatif et concepts clé de Rust
- 16:30 - 17:15 Temps de travail en autonomie

### Installation de Rust

Pour installer Rust on va utiliser le projet [Rustup.rs](https://rustup.rs/),

### Les bases

Pour assimiler les bases je vous propose de faire des exercices en autonomie pour intégrer la syntaxe
et les concepts de base du langage.

On va commencer avec le projet rustlings <https://github.com/rust-lang/rustlings/> jusque
aux `struct`.

Vous pouvez l'installer avec la commande: `curl -L https://git.io/rustlings | bash`

#### Focus sur les struct `Point2D`

Écrire une struct `Point2D` avec deux champs `x` et `y` de type `i32`,
écrire une fonction new et add dans un bloc `impl` voir ce [chapitre sur les methodes](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) du livre de Rust.

- `new` aura la signature suivante: `fn new (x: i32, y: i32) -> Point2D`
- `add` aura la signature suivante: `fn add(&self, a: &Point2D) -> Point2D`

Questions0: Si `add` avait eu la signature `fn add(self, a: Point2D) -> Point2D` quel
problème aurions eu? (Tentez de l'écrire et la tester avec cette signature en
realisant plusieurs additions avec le même point.



## Rust en ligne

Editeur rust en ligne [play.rust-lang.org](https://play.rust-lang.org/)

## Ressources pedagogiques

- Livre officiel : [The Rust programming Language](https://doc.rust-lang.org/book/)
- Tutoriel communautaire en français de Guillaume Gomez https://blog.guillaume-gomez.fr/Rust

## Ressources optionnelles

- Programming Rust Jim Blandy Oreally
