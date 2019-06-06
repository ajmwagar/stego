![Stego](./img/logo.png)

[![Crates.io](https://img.shields.io/crates/v/stego.svg)](https://crates.io/crates/stego)
[![stego](https://docs.rs/stego/badge.svg)](https://docs.rs/stego)
[![Build Status](https://travis-ci.org/ajmwagar/stego.svg?branch=master)](https://travis-ci.org/ajmwagar/stego)
[![dependency status](https://deps.rs/repo/github/ajmwagar/stego/status.svg)](https://deps.rs/repo/github/ajmwagar/stego)



*stego* is a steganographic swiss army knife.

## Features

- Cross platform (MacOS, Windows, Linux)
- Encoding and decoding of images/text/binary files into audio/photo/movie files
- Fast and nearly undetectable encoding.
- Smart `stdin`/`stdout` detection (try piping to `stego` instead of using `--txt`)
- lossless decoding of data
- Simple, stateless CLI
- Zero system-dependencies (standalone binary) 

## ⚒ Usage

```bash

# Simple encoding

# Encodes the message "Hello, Stego!" into the provided image
stego encode text --input image.png --output encoded-image.png --txt "Hello, Stego\!" 

# Simple decoding

# decodes and prints out the encoded message ("Hello, Stego!") hidden in the provided image
stego decode text --input encoded-image.png 

# Stdin detection
echo "Hello, Stego\!" | stego encode text --input image.png --output encoded-image.png

# Example

# encodes contents of "secret" into hostimage.png and saves as output.png
cat secret | stego encode text -i hostimage.png -o output.png 

# decodes and prints contents of "secret"
stego decode text -i output.png

# Help
stego --help
stego encode --help
stego decode --help
```


## 📦 Installation

```bash
cargo install stego
```

OR

```bash
git clone https://github.com/ajmwagar/stego
cd stego
cargo install --path ./ --force
```

## 🚥 Roadmap

- [x] CLI
- [x] Encoding / Decoding of text
- [x] Encoding / Decoding of images **(currently broken see [#5](https://github.com/ajmwagar/stego/issues/5))**
- [x] Encoding / Decoding of binary files
- [x] Add logging
- [ ] Better error handling/messages
- [ ] Add file encryption
- [ ] Add file compression
- [ ] CI/Test suite
- [ ] Trait based API for custom datatypes
- [ ] [bincode](https://github.com/servo/bincode) support
- [ ] Encoding / Decoding of audio files
- [ ] Encoding / Decoding of video files
- [ ] Jurassic Park
- [ ] Another mass extinction
- [ ] ???
