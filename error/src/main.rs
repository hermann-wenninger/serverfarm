use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open(r"C:\Users\Dell\Documents\a_work\serverfarm\error\target\debug\file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("Dateiinhalt: {}", contents);
    Ok(())
}