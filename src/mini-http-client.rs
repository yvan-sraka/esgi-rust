gse std::env;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var("GIPHY_API_KEY")?;
    let url = format!("https://api.giphy.com/v1/gifs/random?api_key={}", api_key);
    let resp = reqwest::blocking::get(&url)?
        .text()?;
    println!("{:#?}", resp);
    Ok(())
}
