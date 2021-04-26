# [Workshop] Super Mastermind

Ready for a little Rust _"hands-on"_ session 💪 ?!

We will create a [Super Mastermind](https://en.wikipedia.org/wiki/Mastermind_(board_game)) CLI game.

If you don't know at all this game, you can check the [WikiHow](https://www.wikihow.com/Play-Mastermind) how-to-play page.

## Mandatory

### Level 1

Create a `super-mastermind` binary with cargo:

```shell
cargo new super-mastermind
```

### Level 2

Run it and display `Hello Rust!` on screen:

```shell
cargo run
```

### Level 3

Create an `enum Color` with 8 fancy colors.

### Level 4

Create a `let guess: vector<Color> = ...` representing a combination of 5 colors.

### Level 5

Debug printing your `guess` vector using `{:?}` (`#[derive(Debug)` will help).

### Level 6

Write your own function `fancy_print_guess(guess: &[Color])` to display combination (for e.g. represent each color with a different capital letter):

```rust
fancy_print_guess(&[Color::Red, Color::Red, Color::Blue, Color::Yellow, Color::Green]);
// -> e.g. ouput `RRBYG`
```

### Level 7

Take advantage of [ansi_term](https://docs.rs/ansi_term/) crate to improve your function with [ANSI](https://en.wikipedia.org/wiki/ANSI_escape_code) colors, add these lines to your `Cargo.toml`:

```
[dependencies]
ansi_term = "0.12.1"
```

### Level 8

Make an infinite `loop` which at each turn: read a string and print it!

### Level 9 🚨

Parse the content of the string read into a `Vec<Color>` and display a nice error message in case of wrong input.

### Level 10

Tweaks our infinite loop game:

- Count the number of guess of the user (it does not increase in case of not well-formed input)
- End the program with a congratulation message if the user guess well the hidden combination

### Level 11

Help the user with a function _(computed and displayed at each guess)_:

```rust
fn number_of_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32
```

### Level 12

...and a function:

```rust
fn number_of_not_well_placed_pawns(secret: &[Color], guess: &[Color]) -> i32
```

### Level 13

Generate randomly the combination the user has to guess _(since until now it was statically hardcoded)_ with [rand](https://docs.rs/rand/) crate.

### YOU are the MasterMind! 🤯

## Bonus

### Level 14

Now switch roles with computer 🤖: code an IA that guess the combination you have in mind!

### Level 15

What about doing games with friends over the network?

### Level 16

What about a GUI? 3D graphics? No limits to your imagination!

## Submission (Deadline: May 9, 2021)

Submit your work by making a Pull-Request to <https://github.com/yvan-sraka/super-mastermind>
