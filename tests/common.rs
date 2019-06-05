use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use stego::*;
use image::{DynamicImage, ImageBgra8};

fn setup() -> LSBStego {
    let im: DynamicImage = image::open(&Path::new(&"./tests/img/test.png")).unwrap();
    let mut stego = LSBStego::new(im.clone());
    stego
}

#[test]
fn encode_decode_text(){
    let mut stego = setup();

    let msg = String::from("Hello, Stego!");

    let im2 = stego.encode_text(msg.clone());

    let mut stego = LSBStego::from_rgba(im2);

    let decoded = stego.decode_text();

    assert_eq!(msg, decoded);
}

#[test]
fn encode_decode_binary(){
    let mut stego = setup();

    let mut bytes = Vec::new();
    let mut file = File::open(&Path::new(&"./tests/img/beemoviescript")).unwrap();
    file.read_to_end(&mut bytes);

    let im2 = stego.encode_binary(bytes.clone());

    let mut stego = LSBStego::from_rgba(im2);

    let decoded = stego.decode_binary();

    assert_eq!(bytes, decoded);
}
