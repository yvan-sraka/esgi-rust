use std::io::{self, Read, Write};

fn main() -> std::io::Result<()> {
    let stdout = io::stdout();

    // All this to display `>` in the console ;)
    {
        let mut handle = stdout.lock();
        handle.write_all(b"> ")?;
        handle.flush()?
    }

    let mut user_input = String::with_capacity(256);
    io::stdin().read_line(&mut user_input)?;
    Ok(())
}