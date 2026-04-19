# LSB Steganography CLI

This is a command-line tool built in Rust to hide and reveal secret messages in images using the Least Significant Bit (LSB) technique. I developed this project as a companion piece for a post on my blog.

It uses [`clap`](https://crates.io/crates/clap) for parsing CLI arguments and [`image`](https://crates.io/crates/image) for pixel manipulation.

## Requirements

To run this project, you need to have Rust installed. You can get it here: [rust install](https://rust-lang.org/tools/install/)

## How to Use

**1. Hide a message:**
```bash
cargo run -- image.png hide "Your secret message here"
```
*(This will generate a new file named `hidden-image.png`)*

**2. Reveal a hidden message:**
```bash
cargo run -- hidden-image.png show
```

>  You can choose to use other image to hide the message, but make sure it has enough pixels to accommodate the message you want to hide. Each character in the message requires 8 bits, and each pixel can hide 3 bits (one in each color channel).

> Choose png images for better results, as they are lossless and won't degrade the hidden message. Avoid using lossy formats like JPEG, as they may corrupt the hidden data.
