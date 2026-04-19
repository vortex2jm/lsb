use clap::{Parser, Subcommand};
use image::{ImageReader, RgbImage};

#[derive(Debug, Subcommand)]
enum OpMode {
    Hide { message: String },
    Show,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    file_path: String,

    #[command(subcommand)]
    mode: OpMode,
}

fn main() {
    let args = Cli::parse();
    let file_path = args.file_path;

    let mut img = ImageReader::open(&file_path)
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();

    match args.mode {
        OpMode::Hide { message } => {
            println!("Hiding message in image...");
            run_hide(&message, &mut img, &file_path);
        }
        OpMode::Show => {
            println!("Showing hidden message from image...");
            run_show(&mut img);
        }
    }
}

// ==========================================================
fn run_hide(msg: &str, img: &mut RgbImage, file_path: &str) {
    let message: &[u8] = msg.as_bytes();
    let msg_len= message.len() as u16;  // Type casting because usize has 64bits (too large)

    let mut payload: Vec<u8> = Vec::new();
    payload.extend_from_slice(&msg_len.to_be_bytes());  // Inject the message length as the first 2 bytes
    payload.extend_from_slice(message);

    // Store message bits values in a vector
    let mut message_bits: Vec<u8> = Vec::new();
    for byte in payload {
        for i in (0..8).rev() {
            message_bits.push((byte >> i) & 1);
        }
    }

    let mut bits_iter = message_bits.iter();

    // Inject message bits into the image pixels
    for pixel in img.pixels_mut() {
        for i in 0..3 {
            if let Some(bit) = bits_iter.next() {
                pixel[i] = (pixel[i] & 0b1111_1110) | (bit & 1);    // Set the least significant bit
            }
        }
    }

    // Save the modified image
    img.save(format!("hidden-{}", &file_path)).unwrap();
}

// ================================================
fn run_show(img: &mut RgbImage) {
    let mut message_bits: Vec<u8> = Vec::new();

    // Extract the least significant bits from the image pixels
    for pixel in img.pixels() {
        for i in 0..3 {
            message_bits.push(pixel[i] & 1);    // Get the least significant bit
        }
    }

    // Convert bits back to bytes
    let mut message_bytes: Vec<u8> = Vec::new();
    for chunk in message_bits.chunks(8) {
        let mut byte = 0u8;
        for (i, bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }
        message_bytes.push(byte);
    }

    // Message length is stored in the first 2 bytes
    let mut len_array = [0u8; 2];
    len_array.copy_from_slice(&message_bytes[0..2]);
    let msg_len = u16::from_be_bytes(len_array) as usize;

    // Convert message bytes to string
    if let Ok(message) = String::from_utf8(message_bytes[2..2 + msg_len].to_vec()) {
        println!("Hidden message: {}", message);
    } else {
        println!("Failed to decode hidden message.");
    }
}
