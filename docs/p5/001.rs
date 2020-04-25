use std::io::{self};

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut user_input = String::with_capacity(256);
    // On prend une référence mutable
    stdin.read_line(&mut user_input)?;
    // `?` sert à « propager l'erreur » dans la fonction appellante
    // c'est mieux que de crash avec un unwrap ou expect ;)
    Ok(())
}