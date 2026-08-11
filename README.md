# Hauffman 🦀

A lightweight CLI tool for **lossless text file compression and decompression** using **Huffman Coding**, written in idiomatic Rust.

---

##  Features

- **Huffman Coding Algorithm**: Builds frequency trees to compress `.txt` files efficiently.
- **Zero-Copy CLI Parsing**: Implements Rust lifetime parameters (`'a`) to parse command-line arguments with zero extra heap allocations.
- **Panic-Safe Argument Matching**: Uses Rust's slice pattern matching (`[_, flag, filepath, ..]`) for safe bounds checking.
- **Robust Error Handling**: Implements `std::error::Error` and `Display` traits for clean error reporting on invalid flags, missing arguments, or I/O failures.

---

## Getting Started

### Prerequisites

Ensure you have **Rust** and **Cargo** installed. You can check with:

```bash
rustc --version
cargo --version
```

If not installed, get Rust from [rustup.rs](https://rustup.rs/).

---

## Usage

### 1. Encoding a File (`.txt`  => `.en.hauff`)

To compress a text file, pass the `-en` flag followed by the path to your `.txt` file:

```bash
cargo run -- -en sample.txt
```

**Output:**
```text
Encoding 'sample.txt'...
Success! File encoded to: sample.en.hauff
```

---

### 2. Decoding a File (`.hauff` → `.den.hauff`)

To decompress an encoded file back to plain text, pass the `-den` flag followed by the path to your `.hauff` file:

```bash
cargo run -- -den sample.en.hauff
```

**Output:**
```text
Decoding 'sample.en.hauff'...
Success! File decoded to: sample.den.hauff
```

---

## Error Handling Examples

| Command | Result |
| :--- | :--- |
| `cargo run` | `Hauffman CLI - Missing arguments!` |
| `cargo run -- -xyz file.txt` | `Unknown flag used. Use '-en' to encode or '-den' to decode.` |
| `cargo run -- -en image.png` | `File is not valid. Should be either .hauff or .txt file` |

---

## License

Distributed under the MIT License. Feel free to use and modify!
