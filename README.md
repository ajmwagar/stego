![Stego](./img/logo.png)

[![Crates.io](https://img.shields.io/crates/v/stego.svg)](https://crates.io/crates/stego)
[![stego](https://docs.rs/stego/badge.svg)](https://docs.rs/stego)
[![Build Status](https://travis-ci.org/ajmwagar/stego.svg?branch=master)](https://travis-ci.org/ajmwagar/stego)
[![dependency status](https://deps.rs/repo/github/ajmwagar/stego/status.svg)](https://deps.rs/repo/github/ajmwagar/stego)
[![License](https://img.shields.io/crates/l/pbr.svg)](https://github.com/ajmwagar/stego/blob/master/LICENSE.md)




*stego* is a steganographic swiss army knife.

## Features


<!--- Encoding and decoding of images/text/binary files into audio/photo/movie files-->
- Cross platform (MacOS, Windows, Linux)
- Encoding and decoding of images/text/binary files into photos (audio/video coming soon)
- Fast and nearly undetectable encoding (to the human eye).
- Smart `stdin`/`stdout` detection (try piping to `stego` instead of using `--payload`)
- lossless decoding of data
- Simple, stateless CLI
- Zero system-dependencies (standalone binary) 

## ⚒ Usage

```bash

# Text encoding/decoding

# Encodes the message "Hello, Stego!" into the provided image
stego encode text --input image.png --output encoded-image.png --payload "Hello, Stego\!" 

# Decodes and prints out the encoded message ("Hello, Stego!") hidden in the provided image
stego decode text --input encoded-image.png 

# File encoding/decoding

# Encodes the file hidden.docx into the provided image
stego encode file --input image.png --output encoded-image.png --payload hidden.docx 

# Decodes and saves the content to decoded.docx from the provided image
stego decode file --input encoded-image.png --output decoded.docx

# Stdin detection (for text-encoding)
echo "Hello, Stego\!" | stego encode text --input image.png --output encoded-image.png

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

## 🤝 Acknowledgments & Contributors

`stego` wouldn't be possible without:

- Nathan Laha ([@TheDekuTree](https://github.com/TheDekuTree))
- Avery Wagar ([@ajmwagar](https://github.com/ajmwagar))

`stego` was inspired by:
- [`xsv`](https://github.com/BurntSushi/xsv)
- [`LSBPython`](https://github.com/RobinDavid/LSB-Steganography)
