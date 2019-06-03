use std::env;
use std::fs::File;
use std::path::Path;

use image::{GenericImageView, ImageBuffer, DynamicImage, Bgr};

const MASK_ONE_VALUES: &[u8] = &[1,2,4,8,16,32,64,128];
const MASK_ZERO_VALUES: &[u8] = &[254,253,251,247,239,223,191,127];

struct LSBStego  {
    /// Image loaded into Stego
    image: ImageBuffer<Bgr<u8>, Vec<u8>>,
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
            image: im.to_bgr(),
            width,
            height,
            channels: 3,
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
                println!("MaskOne: {}", MASK_ONE_VALUES[self.maskONE as usize]);
                val[self.current_channel] = val[self.current_channel] | MASK_ONE_VALUES[self.maskONE as usize]; // Or with maskONE
            }
            else {
                println!("MaskZero: {}", MASK_ZERO_VALUES[self.maskZERO as usize]);
                val[self.current_channel] = val[self.current_channel] & MASK_ZERO_VALUES[self.maskZERO as usize]; // And with maskZERO
            }

            *val = *val;

            self.next_slot();
        }
        println!("bits: {}", bits);
            

    }

    /// move to the next slot where informations can me mutated
    pub fn next_slot(&mut self) {
        if self.current_channel == self.channels - 1 {
            println!("Reseting Channel");
            self.current_channel = 0;

            if self.current_width == self.width - 1 {
                    println!("Reseting Width");
                self.current_width = 0;

                if self.current_height == self.height - 1 {
                    println!("Reseting Hieght");
                    self.current_height = 0;

                    if MASK_ONE_VALUES[self.maskONE as usize] == 128 {
                        panic!("No available slots remaining (image filled)");
                    }
                    else {
                        println!("Changing Masks");
                        self.maskONE += 1;
                        self.maskZERO += 1;
                    }
                }
                else {
                    println!("Changing Height");
                    self.current_height += 1;
                }
            }
            else {
                println!("Changing Width");
                self.current_width += 1;
            }
        }
        else {
            println!("Changing Channel");
            self.current_channel += 1;
        }
    }

    /// Read a single bit from the image
    fn read_bit(&mut self) -> char {
        let val = self.image.get_pixel(self.current_width, self.current_height)[self.current_channel];
        let val = val & MASK_ONE_VALUES[self.maskONE];
        self.next_slot();
        println!("Reading: {}", val);

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
    
    fn encode_text(&mut self, txt: String) -> ImageBuffer<Bgr<u8>, Vec<u8>> {
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

    fn decode_text(&mut self) -> String {
        let size = self.read_bits(16);
        let l = u32::from_str_radix(&size, 2).unwrap();

        let mut txt = String::new();

        for _ in 0..l {
            let tmp = self.read_byte();
            txt.push(u32::from_str_radix(&tmp,2).unwrap() as u8 as char);
        }

        txt
    }
}


fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let im: DynamicImage = image::open(&Path::new(&file)).unwrap();

    // // The dimensions method returns the images width and height
    // println!("dimensions {:?}", im.dimensions());

    // // The color method returns the image's ColorType
    // println!("{:?}", im.color());

    let mut stego = LSBStego::new(im.clone());

    // print!("Hidden: {}",stego.decode_text());

    let im2 = stego.encode_text("9999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999".to_string());

    // Write the contents of this image to the Writer in PNG format.
    im2.save(&Path::new(&format!("output-{}",file)));

}
