extern crate iron;

use iron::prelude::*;
use iron::status;

use iron::mime::{Mime, TopLevel, SubLevel};

// Projet: 
// Écrire un guess the number avec un mini serveur web et un client HTML;
// avec le framework iron.
// Inspiration: Livre : Programming rust chapitre tour of rust.

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mime_html: Mime = "text/html; charset=utf-8".parse().unwrap();
    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime_html);
    response.set_mut(r#"
<title>Guess the number game</title>
<form action="/gcd" method="post">
    <input type="number" name="guess"/>
    <button type="submit">Send number</button>
</form>
"#);
    Ok(response)
}
