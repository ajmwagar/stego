//! Stego library
//! A steganography swiss army knife

use image::{GenericImageView, RgbaImage, DynamicImage, Rgba, Pixel};

const MASK_ONE_VALUES: &[u8] = &[1,2,4,8,16,32,64,128];
const MASK_ZERO_VALUES: &[u8] = &[254,253,251,247,239,223,191,127];

pub struct LSBStego  {
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
    mask_one: usize,
    /// Current index in the MASK_ZERO_VALUES
    mask_zero: usize,

}

impl LSBStego {

    /// Create a new LSBStego instance by taking in a DynamicImage
    pub fn new(im: DynamicImage) -> Self {
        let (width, height) = im.dimensions();


        LSBStego {
            image: im.to_rgba(),
            width,
            height,
            channels: <Rgba<u8> as Pixel>::channel_count() as usize,
            current_height: 0,
            current_width: 0,
            current_channel: 0,
            mask_one: 0,
            mask_zero: 0
        }
    }

    /// Create a new LSBStego instance by taking in a DynamicImage
    pub fn from_rgba(im: RgbaImage) -> Self {
        let (width, height) = im.dimensions();


        LSBStego {
            image: im,
            width,
            height,
            channels: <Rgba<u8> as Pixel>::channel_count() as usize,
            current_height: 0,
            current_width: 0,
            current_channel: 0,
            mask_one: 0,
            mask_zero: 0
        }
    }

    // /// Returns the size of the loaded image
    // fn get_size(&self) -> u32 {
    //     self.height * self.width
    // }

    /// Returns the mask value of the current maskONE index
    pub fn get_mask_one(&self) -> usize {
        MASK_ONE_VALUES[self.mask_one as usize] as usize
    }

    /// Returns the mask value of the current maskZERO index
    pub fn get_mask_zero(&self) -> usize {
        MASK_ZERO_VALUES[self.mask_zero as usize] as usize
    }

    /// Put a string of binary_values into `self.image`
    pub fn put_binary_value(&mut self, bits: String) {
        for c in bits.chars() {
            // Get pixel value
            let val = self.image.get_pixel_mut(self.current_width, self.current_height);

            if c == '1' {
                val[self.current_channel] = val[self.current_channel] | MASK_ONE_VALUES[self.mask_one as usize]; // Or with maskONE
            }
            else {
                val[self.current_channel] = val[self.current_channel] & MASK_ZERO_VALUES[self.mask_zero as usize]; // And with maskZERO
            }

            *val = *val;

            self.next_slot();
        }
            

    }

    /// move to the next slot where information can me mutated
    pub fn next_slot(&mut self) {
        if self.current_channel == self.channels - 1 {
            self.current_channel = 0;

            if self.current_width == self.width - 1 {
                self.current_width = 0;

                if self.current_height == self.height - 1 {
                    self.current_height = 0;

                    if MASK_ONE_VALUES[self.mask_one as usize] == 128 {
                        panic!("No available slots remaining (image filled)");
                    }
                    else {
                        self.mask_one += 1;
                        self.mask_zero += 1;
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
        let val = val & MASK_ONE_VALUES[self.mask_one];
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

    /// Returns a binary string in byte size of a given integer
    fn byte_value(&self, val: usize) -> String {
        self.binary_value(val, 8)
    }
    
    /// Returns the binary of a given integer in the length of `bitsize`
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
            let byte_value = self.byte_value(c as usize);
            self.put_binary_value(byte_value)
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
        let im = im.to_bgra();
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
                    // println!("Chan: {}/{}, Val: {}", chan, channels, val);
                    self.put_binary_value(self.byte_value(val as usize));
                }

            }
        }

        self.image.clone()
    }

    /// Decodes a hidden image from another image
    pub fn decode_image(&mut self) -> RgbaImage {
        let channels = <Rgba<u8> as Pixel>::channel_count() as u32;

        let width = u32::from_str_radix(&self.read_bits(16), 2).unwrap();
        let height = u32::from_str_radix(&self.read_bits(16), 2).unwrap();
        let mut unhideimg = image::RgbaImage::new(width, height);

        for h in 0..height {
            for w in 0..width {
                for chan in 0..channels {
                    let val = unhideimg.get_pixel_mut(w,h);
                    val[chan as usize] = u8::from_str_radix(&self.read_byte(), 2).unwrap();
                    // println!("Chan: {}/{}, Val: {}", chan, channels, val[chan as usize]);
                }
            }
        }

        unhideimg
    }

    /// Encodes a binary file into the image
    pub fn encode_binary(&mut self, data: Vec<u8>) -> RgbaImage {
        let length = data.len();

        if self.width*self.height*(self.channels as u32) < length as u32 + 64 {
            panic!("Carrier image not big enough to hold hidden file");
        }

        self.put_binary_value(self.binary_value(length, 64));

        for byte in data {
            self.put_binary_value(self.byte_value(byte as usize));
        }

        self.image.clone()
    }

    /// Encodes a binary file into the image
    pub fn decode_binary(&mut self) -> Vec<u8> {
        let length = usize::from_str_radix(&self.read_bits(64), 2).unwrap();
        let mut output: Vec<u8> = Vec::with_capacity(length);

        if self.width*self.height*(self.channels as u32) < length as u32 + 64 {
            panic!("Carrier image not big enough to hold hidden file");
        }

        for _ in 0..length{
            output.push(u8::from_str_radix(&self.read_byte(),2).unwrap());
        }
         
        output

    }
}
