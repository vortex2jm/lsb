use clap::Parser;
use image::ImageReader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,
}

fn main() {    

    let args = Cli::parse();
    let file_path = args.file_path;

    let mut img = ImageReader::open(&file_path)
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();


    let message = "Hello, my name is Joao".as_bytes();
    let mut bits = message.iter()
        .flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1));

    for pixel in img.pixels_mut() {
        for i in 0..3 {
            if let Some(bit) = bits.next() {
                pixel[i] = set_lsb(pixel[i], bit);
            }
        }
    }

    img.save(format!("encoded-{}", &file_path)).unwrap();
}

fn set_lsb(value: u8, bit: u8) -> u8 {
    (value & 0b1111_1110) | (bit & 1)
}
