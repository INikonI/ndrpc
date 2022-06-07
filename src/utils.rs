use std::io::{stdin, Error};

pub fn prompt(text: &str) -> Result<String, Error> {
    println!("{}", text);
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}
