use std::env;

use rusty_tesseract::{Image, Args};

fn main() {
    let args : Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [path-to-image]",&args[0]);
        return;
    }

    let path_to_image = &args[1];
    let img = Image::from_path(path_to_image).unwrap();
    let default_args = Args::default();
    let text = rusty_tesseract::image_to_string(&img, &default_args).unwrap();

    println!("Recognized Text:\n{}", text);
}
