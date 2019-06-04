#[macro_use]
extern crate structopt;
use structopt::StructOpt;

use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::prelude::*;

use image::{GenericImageView, RgbaImage, DynamicImage, Rgba, Pixel};

const MASK_ONE_VALUES: &[u8] = &[1,2,4,8,16,32,64,128];
const MASK_ZERO_VALUES: &[u8] = &[254,253,251,247,239,223,191,127];

struct LSBStego  {
    /// Image loaded into Stego
    image: RgbaImage,
    /// Hieght of loaded image
    height: u32,
    /// Width of loaded image
    width: u32,

    /// Number of channels in loaded image
    channels: usize,

    /// Current width position
    current_width: u32,
    /// Current height position
    current_height: u32,
    /// Current channel position
    current_channel: usize,

    /// Current index in the MASK_ONE_VALUES
    maskONE: usize,
    /// Current index in the MASK_ZERO_VALUES
    maskZERO: usize,

}

impl LSBStego {

    pub fn new(im: DynamicImage) -> Self {
        let (width, height) = im.dimensions();

        LSBStego {
            image: im.to_rgba(),
            width,
            height,
            channels: 4,
            current_height: 0,
            current_width: 0,
            current_channel: 0,
            maskONE: 0,
            maskZERO: 0
        }
    }

    /// Returns the size of the loaded image
    pub fn get_size(&self) -> u32 {
        self.height * self.width
    }

    /// Returns the mask value of the current maskONE index
    pub fn get_mask_one(&self) -> usize {
        MASK_ONE_VALUES[self.maskONE as usize] as usize
    }

    /// Returns the mask value of the current maskZERO index
    pub fn get_mask_zero(&self) -> usize {
        MASK_ZERO_VALUES[self.maskZERO as usize] as usize
    }

    pub fn put_binary_value(&mut self, bits: String) {
        for c in bits.chars() {
            // Get pixel value
            let mut val = self.image.get_pixel_mut(self.current_width, self.current_height);

            if c == '1' {
                val[self.current_channel] = val[self.current_channel] | MASK_ONE_VALUES[self.maskONE as usize]; // Or with maskONE
            }
            else {
                val[self.current_channel] = val[self.current_channel] & MASK_ZERO_VALUES[self.maskZERO as usize]; // And with maskZERO
            }

            *val = *val;

            self.next_slot();
        }
            

    }

    /// move to the next slot where informations can me mutated
    pub fn next_slot(&mut self) {
        if self.current_channel == self.channels - 1 {
            self.current_channel = 0;

            if self.current_width == self.width - 1 {
                self.current_width = 0;

                if self.current_height == self.height - 1 {
                    self.current_height = 0;

                    if MASK_ONE_VALUES[self.maskONE as usize] == 128 {
                        panic!("No available slots remaining (image filled)");
                    }
                    else {
                        self.maskONE += 1;
                        self.maskZERO += 1;
                    }
                }
                else {
                    self.current_height += 1;
                }
            }
            else {
                self.current_width += 1;
            }
        }
        else {
            self.current_channel += 1;
        }
    }

    /// Read a single bit from the image
    fn read_bit(&mut self) -> char {
        let val = self.image.get_pixel(self.current_width, self.current_height)[self.current_channel];
        let val = val & MASK_ONE_VALUES[self.maskONE];
        self.next_slot();

        if val > 0 { '1' } else { '0' }
        
    }

    /// Read a byte of the image
    fn read_byte(&mut self) -> String {
        self.read_bits(8)
    }

    /// Read n bits from an image
    fn read_bits(&mut self, n: u32) -> String {
        let mut bits = String::with_capacity(n as usize);

        for _ in 0..n {
            bits.push(self.read_bit())
        }

        bits
    }

    fn byteValue(&self, val: usize) -> String {
        self.binary_value(val, 8)
    }
    
    fn binary_value(&self, val: usize, bitsize: usize) -> String {
        let mut binval = String::with_capacity(bitsize);
        binval.push_str(&format!("{:b}", val));

        if binval.len() > bitsize {
            panic!("binary value larger than the expected size");
        }
        
        while binval.len() < bitsize {
            binval.insert(0, '0');
        }
        binval

    }
    
    /// Encodes a text message into an image
    pub fn encode_text(&mut self, txt: String) -> RgbaImage {
        // Length coded on 2 bytes
        let binl = self.binary_value(txt.len(), 16);
        self.put_binary_value(binl);
        for c in txt.chars() {
            let byteValue = self.byteValue(c as usize);
            self.put_binary_value(byteValue)
        }

        // Return the new image
        self.image.clone()
    }

    /// Decodes a hidden message from an image
    pub fn decode_text(&mut self) -> String {
        let size = self.read_bits(16);
        let l = u32::from_str_radix(&size, 2).unwrap();

        let mut txt = String::new();

        for _ in 0..l {
            let tmp = self.read_byte();
            txt.push(u32::from_str_radix(&tmp,2).unwrap() as u8 as char);
        }

        txt
    }

    /// Encodes an image into another image
    pub fn encode_image(&mut self, im: DynamicImage) -> RgbaImage {
        let (width, height) = im.dimensions();

        let channels = <Rgba<u8> as Pixel>::channel_count() as u32;

        if self.width * self.height * (self.channels as u32) < width * height * channels {
            panic!("Carrier image not big enough to hold hidden image");
        }

        let binw = self.binary_value(width as usize, 16);
        let binh = self.binary_value(height as usize, 16);

        self.put_binary_value(binw);
        self.put_binary_value(binh);

        for h in 0..height{
            for w in 0..width {
                for chan in 0..channels {
                    let val = im.get_pixel(w, h)[chan as usize];
                    self.put_binary_value(self.byteValue(val as usize));
                }

            }
        }

        self.image.clone()
    }

    fn decode_image(&mut self) -> RgbaImage {
        let channels = <Rgba<u8> as Pixel>::channel_count() as u32;

        let width = u32::from_str_radix(&self.read_bits(16), 2).unwrap();
        let height = u32::from_str_radix(&self.read_bits(16), 2).unwrap();
        let mut unhideimg = image::RgbaImage::new(width, height);

        for h in 0..height {
            for w in 0..width {
                for chan in 0..channels {
                    let val = unhideimg.get_pixel_mut(w,h);
                    val[chan as usize] = u8::from_str_radix(&self.read_byte(), 2).unwrap();
                }
            }
        }

        self.image.clone()
    }
}

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
