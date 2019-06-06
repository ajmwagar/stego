// #[macro_use] extern crate structopt;
#[macro_use] extern crate log;
use log::{LevelFilter};
use atty::Stream;

use structopt::clap::arg_enum;
use structopt::StructOpt;

use stego::*;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::error::Error;

use image::{DynamicImage};

arg_enum! {
    #[derive(Debug)]
    enum DataType {
        Image,
        Text,
        File
    }
}


#[derive(StructOpt, Debug)]
#[structopt(name = "stego", about = "Stegnography at it's finest")]
enum StegoCLI {
    #[structopt(name = "encode")]
    /// Encoding command
    Encode {
        #[structopt(short = "i", long = "input", parse(from_os_str))]
        /// Input image
        input: PathBuf,
        #[structopt(short = "o", long = "output", parse(from_os_str))]
        /// File to save modified image as
        output: PathBuf,

        #[structopt(raw(possible_values = "&DataType::variants()", case_insensitive = "true"))]
        /// Type of data to embed in image
        dtype: DataType, 

        #[structopt(short = "p", long = "payload")]
        /// Data to embed in image (either text message or path)
        payload: Option<String>,

    },
    #[structopt(name = "decode")]
    Decode {
        #[structopt(short = "i", long = "input", parse(from_os_str))]
        /// Input image to decode
        input: PathBuf,

        #[structopt(short = "o", long = "output", parse(from_os_str))]
        /// Path to save hidden image/file to
        output: Option<PathBuf>,

        #[structopt(raw(possible_values = "&DataType::variants()", case_insensitive = "true"))]
        /// Type of data to embed in image
        dtype: DataType, 
    },
}

fn main() -> Result<(), Box<dyn Error>> {

    if atty::is(Stream::Stdout) {
        print_header();

        let mut builder = pretty_env_logger::formatted_timed_builder();

        // .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
        builder.filter(None, LevelFilter::Info).init();
    }

    let opt = StegoCLI::from_args();

    match opt {
        StegoCLI::Encode{ input, output, dtype, payload } => {
            // Use the open function to load an image from a Path.
            // ```open``` returns a dynamic image.
            let im: DynamicImage = image::open(&Path::new(&input))?;
            let mut stego = LSBStego::new(im.clone());

            let mut im2;

            info!("Loading host image: {}", &input.into_os_string().into_string().unwrap());

            match dtype {
                DataType::File => {
                    let path = payload.unwrap();
                    let mut bytes = Vec::new();
                    
                    info!("Loading binary file {}", &path);

                    let mut file = File::open(&Path::new(&path))?;

                    file.read_to_end(&mut bytes)?;

                    info!("Encoding to host image...");


                    im2 = stego.encode_binary(bytes);

                },
                DataType::Image => {
                    let path = payload.unwrap();
                    info!("Loading hidden image {}", &path);

                    let pim: DynamicImage = image::open(&Path::new(&path))?;


                    info!("Encoding to host image...");

                    im2 = stego.encode_image(pim);

                },
                DataType::Text => {
                    if payload != None {
                        info!("Encoding text paylod to host image...");
                        im2 = stego.encode_text(payload.unwrap());
                    }
                    else {
                        warn!("No payload specified... Reading from stdin");

                        let mut msg = String::new();
                        std::io::stdin().read_to_string(&mut msg)?;

                        info!("Encoding to host image...");
                        im2 = stego.encode_text(msg);
                    }
                }
            } 

            info!("Saving file to {:?}", output);

            im2.save(&Path::new(&output))?;

            info!("Done!");

            Ok(())
        },
        StegoCLI::Decode { input, output, dtype} => {
            // Use the open function to load an image from a Path.
            // ```open``` returns a dynamic image.
            let im: DynamicImage = image::open(&Path::new(&input))?;
            let mut stego = LSBStego::new(im.clone());

            match dtype {
                DataType::File => {
                    info!("Saving file to {:?}", output);

                    let mut file = File::create(&Path::new(&output.unwrap()))?;


                    file.write_all(&stego.decode_binary())?;

                    Ok(())

                },
                DataType::Image => {
                    // TODO: Fix this
                    warn!("Image decoding is currently broken (see https://github.com/ajmwagar/stego/issues/5)");

                    let im2 = stego.decode_image();

                    info!("Saving file to {:?}", output);

                    im2.save(&Path::new(&output.unwrap()))?;

                    Ok(())

                },
                DataType::Text => {
                    // TODO Support hidden image / hiddenfile
                    print!("{}",stego.decode_text());

                    Ok(())
                }
            } 


        }

    }
}

fn print_header() {
    println!(r"
     _                   
 ___| |_ ___  __ _  ___  
/ __| __/ _ \/ _` |/ _ \ 
\__ \ ||  __/ (_| | (_) |
|___/\__\___|\__, |\___/ 
             |___/       
a steganographic swiss army knife
=========================
Created by: Avery Wagar
")
}
