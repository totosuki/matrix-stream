use std::{
    io::{stdin, stdout, Result, Write},
};

use image::{DynamicImage, ImageReader, ImageResult};

fn main() -> Result<()> {
    println!("[sender] start");

    let mut path = String::new();
    print!("File path : ");
    stdout().flush()?;
    stdin().read_line(&mut path);
    path = path.trim().to_string();
    
    let data = load_image(path);

    Ok(())
}

fn load_image(path: String) -> DynamicImage {
    let tmp = ImageReader::open(format!("{}", path)).unwrap();
    tmp.decode().unwrap()
}