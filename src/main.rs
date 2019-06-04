#[macro_use]
extern crate structopt;
use structopt::StructOpt;

use stego::*;

use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;

use image::{GenericImageView, RgbaImage, DynamicImage, Rgba, Pixel};


#[derive(StructOpt, Debug)]
#[structopt(name = "stego", about = "Stegnography at it's finest")]
enum StegoCLI {
    #[structopt(name = "encode")]
    Encode {
        #[structopt(short = "i", long = "input", parse(from_os_str))]
        /// Input image
        input: PathBuf,
        #[structopt(short = "o", long = "output", parse(from_os_str))]
        /// File to save modified image as
        output: PathBuf,

        #[structopt(short = "t", long = "txt")]
        /// Text message to embed in image
        hiddentext: Option<String>,

        #[structopt(long = "image", parse(from_os_str))]
        /// Image to embed in host image
        hiddenimage: Option<PathBuf>,

        #[structopt(short ="f", long = "file", parse(from_os_str))]
        /// File to embed in host image
        hiddenfile: Option<PathBuf>,
    },
    #[structopt(name = "decode")]
    Decode {
        #[structopt(short = "i", long = "input", parse(from_os_str))]
        /// Input image to decode
        input: PathBuf,

        #[structopt(long = "image", parse(from_os_str))]
        /// Path to save hidden image to
        hiddenimage: Option<PathBuf>,

        #[structopt(short ="f", long = "file", parse(from_os_str))]
        /// Path to save file to
        hiddenfile: Option<PathBuf>,
    },
}

fn main() {
    let opt = StegoCLI::from_args();

    match opt {
        StegoCLI::Encode{ input, output, hiddentext, hiddenfile, hiddenimage } => {
            // Use the open function to load an image from a Path.
            // ```open``` returns a dynamic image.
            let im: DynamicImage = image::open(&Path::new(&input)).unwrap();
            let mut stego = LSBStego::new(im.clone());

            let mut im2;

            if hiddentext != None {
                im2 = stego.encode_text(hiddentext.unwrap());
            }
            else {
                let mut msg = String::new();
                std::io::stdin().read_to_string(&mut msg);

                im2 = stego.encode_text(msg);
            }

            println!("Saving file to {:?}", output);

            im2.save(&Path::new(&output));

        },
        StegoCLI::Decode { input, hiddenimage, hiddenfile } => {
            // Use the open function to load an image from a Path.
            // ```open``` returns a dynamic image.
            let im: DynamicImage = image::open(&Path::new(&input)).unwrap();
            let mut stego = LSBStego::new(im.clone());

            // TODO Support hidden image / hiddenfile
            print!("{}",stego.decode_text());

        }

    }


    // let file = if env::args().count() == 3 {
    //     env::args().nth(2).unwrap()
    // } else {
    //     panic!("Please enter a file")
    // };


    // // // The dimensions method returns the images width and height
    // // println!("dimensions {:?}", im.dimensions());

    // // // The color method returns the image's ColorType
    // // println!("{:?}", im.color());


    // if env::args().nth(1).unwrap() == "decode" {
    // }
    // else if env::args().nth(1).unwrap() == "encode" {
    // }



}
