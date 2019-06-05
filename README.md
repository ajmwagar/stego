![Stego](./img/logo.png)

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
stego encode --input image.png --output encoded-image.png --txt "Hello, Stego\!" # Encodes the message "Hello, Stego!" into the provided image

# Simple decoding
stego decode --input encoded-image.png # prints out the encoded message ("Hello, Stego!") hidden in the provided image

# Stdin detection
echo "Hello, Stego\!" | stego encode --input image.png --output encoded-image.png

# Example
cat secret | stego encode -i hostimage.png -o output.png # encodes contents of secret into hostimage.png
stego decode -i output.png # prints contents of secret

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
- [x] Encoding / Decoding of images 
- [x] Encoding / Decoding of binary files
- [ ] Better error handling/messages
- [ ] CI/Test suite
- [ ] Trait based API for custom datatypes
- [ ] [bincode](https://github.com/servo/bincode) support
- [ ] Encoding / Decoding of audio files
- [ ] Encoding / Decoding of video files
- [ ] Jurassic Park
- [ ] Another mass extinction
- [ ] ???
