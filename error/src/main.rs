use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("./file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("Dateiinhalt: {}", contents);
    Ok(())
}