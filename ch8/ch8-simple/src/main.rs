use std::error::Error;
use reqwest;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.rustinaction.com/";
    let response = reqwest::blocking::get(url)?;
    let content = response.text()?;

    println!("{}", content);

    Ok(())
}