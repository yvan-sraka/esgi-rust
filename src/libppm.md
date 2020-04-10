# Big project guidelines: a not so simple PPM image manipulation library
 
Goals: Manipulating IO, Memory, Concurrent programming with threads and exposing Rust function
through Foreign Function Interface.
 
We will use the [Portable Pixel Map](https://en.wikipedia.org/wiki/Netpbm_format) in binary format.
 
## Lvl 0 - Warmup
 
- Create a new `ppm` Rust project as a library
- Make a little readme present your team
- Complete a little bit your Cargo.toml to add yourselves as author
- Make sure that `cargo build` is working
 
## Lvl 1 - Struct
 
- Create a nice custom Rust data-structure `Image` to handle 24bits images.
 
> Create first a `Pixel` structure to encode an RGB color.
 
In a PPM file pixels il take 24bits; 8 bits (octet) par color:
 
- 8 for red
- 8 for green
- 8 for blue
 
### Epic Functions for an Epic `Pixels`
 
Because test are nices: Write a test for all the functions [doc](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
Also, write a doc in rust style *Prof! I don't how to write doc!* [short awnser](https://doc.rust-lang.org/rust-by-example/meta/doc.html); [long answer](https://doc.rust-lang.org/1.30.0/book/2018-edition/ch14-02-publishing-to-crates-io.html?highlight=document#making-useful-documentation-comments)
TL;DR writes how to use the function if it panic says it! make an example in the doc!
 
Warmups functions:
 
- Write a block `impl Pixel` for next questions
- Write a nice constructor `fn new(red: u8, green: u8, blue: u8 -> Self`
- (Optional) Derive `Clone` and `Copy` because a Pixel is a tiny type who fits in a register.
- Make a `fn display(self) -> String` to render a pixel in terminal
- Carve a function `fn invert(&mut self)` that inverts a pixel. Advise think [bitwise](https://doc.rust-lang.org/std/ops/trait.Not.html)*!* ;)
 
Not so warmup:
 
- Make a function `fn eq(self, other: Pixel) -> bool`
- Rewrite your code of `eq` function to implement `PartialEq` [Doc PartialEq](https://doc.rust-lang.org/std/cmp/trait.Eq.html)!
 
Advice: Read the doc again! :)
 
Need some engagement:
 
- Make a function `grayscale` who convert an RGB pixel to a grayscale pixel
 
## Lvl 2 - Image manipulation
 
Write a little `struct Image` that represents a struct you might need the following fields:
 
- `Vec<Pixel>` to represent the pixels buffer
- `heigth` with type `usize`
- `width` with type `usize`
- Maybe other fields?
 
Now write some function to manipulate images!
A function `fn new_with_file(filename: &Path) -> Image` that read in text mode a ppm image you might need to write
the `.ppm` file format definition [here](http://netpbm.sourceforge.net/doc/ppm.html).
 
Write in a `impl Image` a function `fn save(filename: &Path)` who saves your struct into a file.
 
An example file in text mode looks like:
 
```text
P3
3 2
255
# The part above is the header
# "P3" means this is an RGB color image in ASCII
# "3 2" is the width and height of the image in pixels
# "255" is the maximum value for each color
# The part below is image data: RGB triplets
255 0   0   # red
0   255 0   # green
0   0   255 # blue
255 255 0   # yellow
255 255 255 # white
0   0   0   # black
```
 
- Create an `invert` function that inverts image colors (reuse your code!)
- Create a `grayscale` function that makes image B&W based on a filter color
 
### Buffering
 
Try to with really big files, improve performances. Maybe this doc pointer can help: [std::io::BufRead](https://doc.rust-lang.org/std/io/trait.BufRead.html)
 
> [Here](https://mega.nz/#!dQNSyASY!Vk6rM8ZqxpbwvSyRFzHdYVB1Rh8p_6yKTDewtUxVe6Q)
> is a big `.ppm` the file of a Mandelbrot fractal for your tests!
>
> You may want to open the file in Rust using a buffer to do the reading of the file and
> image operations in parallel <https://doc.rust-lang.org/std/io/trait.Read.html>
>
> `.ppm` file format has a simple syntax specified here:
> <http://netpbm.sourceforge.net/doc/ppm.html>
>
> The purpose of this level is to code a custom `.ppm` reading function in Rust!
 
### Bonus binary edition
 
- Prepare yourself to read the pixel part in binary mode write a `fn new_with_file_bin(Path filename) -> Image`
 
> The whole purpose of this part is to do these computations using several threads,
> and to develop a strategy to find a good compromise on the number of threads to assign
> (e.g. one thread by pixel is maybe a bit too much ...)
>
> Try to calls these functions from another language!
 
## Lvl 3 - Benchmarks
 
- Add benchmarks tests to measure your performances
 
> Rust have off course a guide on this topic
> <https://doc.rust-lang.org/1.2.0/book/benchmark-tests.html>
 
## Going further
 
### Lvl 4 - Unsafe - Optional C fun with Rust
 
- Create an `readPPM` function to read `.ppm` files
- Create an `writePPM` function to read `.ppm` files
 
> Use PPMA_IO: Portable Pixel Map (ASCII) Files Read and Write Utilities
> <https://people.sc.fsu.edu/~jburkardt/c_src/ppma_io/ppma_io.html>
>
> You may need to call C code in Rust using `unsafe` blocks.
> If you want to know about some dark magics, read <https://doc.rust-lang.org/nomicon/ffi.html>
>
> You can start with a simpler format like `.pbm` or `.pgm`.
>
> You should make a static library and include it,
> [this StackOverflow post](https://stackoverflow.com/questions/43826572/where-should-i-place-a-static-library-so-i-can-link-it-with-a-rust-program)
> could help!
 
### Lvl 5 - FFI for the fun and profit
 
Relatively easy:
 
- Make a `dummy()` function that return `42`
- Call this function in another language like Python using basic Foreign Function Interface FFI
 
_Note:_
> The Rust doc has a really handy page on the subject:
> <https://doc.rust-lang.org/1.2.0/book/rust-inside-other-languages.html>
 
Somewhat harder:
 
- Exposes functions of your `ppm` crate to Python
- Use [PyO3](https://github.com/PyO3/pyo3) and [Maturin](https://github.com/PyO3/maturin) read the README of the projects!

---

Bonjour tout le monde,

J'espère que tout ce passe bien pour vous avec le projet Rust.

Pour aider ceux d'entre vous qui ne sont peut-être pas fluent dans la langue de Shakespeare (by the way, I am not either), en bas de ce mail ce trouve une version compacte du sujet du projet en français !

J'en profite aussi pour récapituler ce que j'attends et évaluerai de votre soutenance : 15 min de présentation + 5 min de questions (par groupe de 3/4 personnes) le Vendredi 14 Février (dernier examen avant de profiter de votre weekend et de std::option<la St Valentin>).

J'attends de vous que vous me montriez une petite démo de votre bibliothèque, que vous m'expliquiez les décisions que vous avez prise dans le développement de celle-ci, les difficultés que vous avez rencontrées, comment vous les avez ou non surmonté, et une très bonne manière d'illustrer cela ce sont des Benchmarks de vos fonctions : <https://doc.rust-lang.org/1.2.0/book/benchmark-tests.html>

En particulier je vous invite à vous poser la question de comment faire les calculs demandés en utilisant plusieurs threads, et développer une stratégie pour trouver un bon compromis sur le nombre de threads à attribuer (par exemple, un thread par pixel est peut-être un peu trop ...), écrivez des tests ! de la documentation !

Tous les bonus sont appréciés, que ce soit des FFI avec Python, un super affichage dans le terminal, une interface web, ou quoi que vous puissiez trouver amusant à faire :)

Enfin, j'attends aussi que le code de votre projet soit soumis comme Pull-request du dépôt <https://github.com/rust-esgi/libppm>

# Projet : une bibliothèque de manipulation d'images PPM

Objectifs : manipuler les E/S, la mémoire, la programmation concurrente avec des threads et exposer des fonctions Rust via des FFI (interface de fonction étrangère).
 
Nous utiliserons le format [Portable Pixel Map](https://en.wikipedia.org/wiki/Netpbm_format) au format texte et binaire.
 
## Hello World
 
- Créer un nouveau projet Rust `ppm` en tant que bibliothèque
- Faites un petit `README` pour présenter votre équipe
- Complétez un peu votre `Cargo.toml` pour vous ajouter comme auteur
- Assurez-vous que `cargo build` fonctionne
 
## Structures de données et fonctions de base
 
Créez une belle structure de données Rust personnalisée `Image` pour gérer les images 24 bits.
 
Créez d'abord une structure `Pixel` pour encoder une couleur RVB.
 
Dans un fichier PPM, les pixels prennent 24 bits; 8 bits (octet) par couleur:
- 8 pour le rouge
- 8 pour le vert
- 8 pour le bleu

Pour s'échauffer:
- Ecrire un bloc `impl Pixel` pour les prochaines questions
- (Facultatif) Écrire un joli constructeur `fn new(red: u8, green: u8, blue: u8 -> Self`
- Dérivez `Clone` et` Copy` (car un Pixel est un type tout petit qui tient dans un registre).
- Faite un trait `std::fmt::Display` pour afficher un pixel dans le terminal
- Écrire une fonction `fn invert(&mut self)` qui inverse un pixel.
 
Un peu plus dur:
- Faire une fonction `fn eq(self, other: Pixel) -> bool`
- Implémenter` PartialEq` <https://doc.rust-lang.org/std/cmp/trait.Eq.html> !
- Faire une fonction `grayscale` qui convertit un pixel RVB en pixel de niveaux de gris

Écrivez des tests pour toutes vos fonctions <https://doc.rust-lang.org/book/ch11-01-writing-tests.html>

Et de la documentation :
- <https://doc.rust-lang.org/rust-by-example/meta/doc.html>
- <https://doc.rust-lang.org/1.30.0/book/2018-edition/ch14-02-publishing-to-crates-io.html?highlight=document#making-useful-documentation-comments>

## Manipulation d'images PPM
 
Écrivez une `struct Image` qui représente une structure dont vous pourriez avoir besoin des champs suivants :
 
- `Vec <Pixel>` pour représenter le tampon de pixels
- `height` avec le type` usize`
- `weight` avec le type` usize`
 
Maintenant, écrivez des fonctions pour manipuler les images !

Une fonction `fn new_with_file(filename: &Path) -> Image` qui lit en mode texte une image PPM 

Le format de fichier `.ppm` a une syntaxe simple spécifiée ici <http://netpbm.sourceforge.net/doc/ppm.html>

Un exemple de fichier en mode texte ressemble à ceci:

    P3
    3 2
    255
    # La partie ci-dessus est l'en-tête
    # "P3" signifie qu'il s'agit d'une image couleur RVB en ASCII
    # "3 2" est la largeur et la hauteur de l'image en pixels
    # "255" est la valeur maximale pour chaque couleur
    # La partie ci-dessous est constituée de données d'image: triplets RVB
    255 0 0 # rouge
    0 255 0 # vert
    0 0 255 # bleu
    255 255 0 # jaune
    255 255 255 # blanc
    0 0 0 # noir

- Créez une fonction `invert` qui inverse les couleurs de l'image (réutilisez votre code!)
- Créer une fonction `grayscale` qui transforme l'image Noir & Blanc en se basant sur une couleur de filtre

Bonus :
- Écrivez dans  `impl Image` une fonction `fn save(filename: &Path)` qui enregistre votre structure dans un fichier.
- Essayer de recoder vos fonctions pour le format PPM binaire (c'est plus simple que vous ne le pensez, et ça peut être une bonne excuse pour ne pas implémenter le format texte) !

## Mise en mémoire tampon
 
Essayez avec de très gros fichiers, améliorez les performances ! <https://doc.rust-lang.org/std/io/trait.BufRead.html>
 
Voici <https://mega.nz/#!dQNSyASY!Vk6rM8ZqxpbwvSyRFzHdYVB1Rh8p_6yKTDewtUxVe6Q> un gros `.ppm` le fichier d'une fractale de Mandelbrot pour vos tests!

Vous voudrez sans doute ouvrir le fichier dans Rust en utilisant un tampon pour faire la lecture du fichier et des opérations d'image en parallèle <https://doc.rust-lang.org/std/io/trait.Read.html>

---

Bonsoir tout le monde,

Comme vous êtes déjà tous bien au courant c'est demain que vous présenterez vos projets Rust, j'en profite ici pour vous donner dans les grandes lignes la grille de notation que j'utiliserai pendant votre oral, afin de limiter les mauvaises surprises éventuelles (15 min de présentation et questions) :

- état d'avancement du projet et implémentation éventuelle de bonus : 10 points
- qualité du code, de ses commentaires / documentation et présence de tests : 5 points
- clarté de la présentation, de la démo du projet et présences de benchmarks : 5 points

Je jetterais bien entendu pendant votre présentation un coup d'œil au code source que vous avez push sur GitHub, je m'attends à ce que celui-ci compile et soit livré avec un README qui explique comment utiliser votre bibliothèque et comment éventuellement lancer des benchmarks / tests.

ATTENTION : Veuillez ne pas oubliez d'indiquer en haut de votre README les noms des membres de votre groupe !


Je m'autorise également à poser en fin de présentation en plus de questions sur votre implémentation, des questions générales sur le langage Rust et autres aspects qu'on aurait vu en cours à certains membres du groupe ! Ce n'est pas pour vous piéger, c'est plus pour donner des points supplémentaires à des groupes où par exemple le temps de parole pendant la présentation n'aurait pas été distribué très équitablement.


<https://en.wikipedia.org/wiki/The_Force#"May_the_Force_be_with_you">

Yvan