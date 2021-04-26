# [Workshop] Rustlings

## Install [rustlings](https://github.com/rust-lang/rustlings) in your path

```shell
cd ~
curl -L https://git.io/install-rustlings | bash
```

## Doing exercises

```shell
cd ~/rustlings
rustlings verify
```

Open `~/rustlings` in your IDE (e.g. with VSCode `code ~/rustlings`) and try to fix `rustc` compiler errors for each file.

In case of emergency, read the corresponding chapter in _the book_: [The Rust Programming Language](https://doc.rust-lang.org/book/)

## Mandatory

| Exercise               | Book Chapter  |
|------------------------|---------------|
| variables              | §3.1          |
| functions              | §3.3          |
| if                     | §3.5          |
| move_semantics         | §4.1          |
| primitive_types        | §4.3          |
| structs                | §5.1          |
| enums                  | §6            |

## Bonus

| Exercise               | Book Chapter  |
|------------------------|---------------|
| modules                | §7.2          |
| collections            | §8.1          |
| strings                | §8.2          |
| error_handling         | §9            |
| generics               | §10           |
| option                 | §10.1         |
| traits                 | §10.2         |
| tests                  | §11.1         |
| standard_library_types | §13.2         |
| threads                | §16.1         |
| macros                 | §19.6         |
| clippy                 | n/a           |
| conversions            | n/a           |

## Submission (Deadline: May 9, 2021)

Submit your work by making a Pull-Request to <https://github.com/yvan-sraka/rustlings>

```shell
cd ~/rustlings
git commit -am "😎"
git remote rm origin
git remote add origin URL_OF_YOUR_FORK
git fetch origin
git branch --set-upstream-to=origin/main
git pull --rebase
git push
```