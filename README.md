# Balatro Handy Odds Library

## Overview

This repository hosts the Rust library for **Balatro Handy Odds**, a tool designed to calculate poker odds and statistics. The core functionality of this library is compiled to WebAssembly (wasm) to enable its use in web browsers, facilitating a seamless integration into web-based poker analysis tools.

## Getting Started

If you're using nix, you can just `nix develop`. Otherwise, you can look inside `flake.nix` to see the packages being utilized for the development environment, which includes things such as `wasm-pack`.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Building from Source

To compile the library to wasm, navigate to the project root and run:

```sh
wasm-pack build --target web .
```
This command generates the wasm binaries along with the necessary JavaScript bindings in the pkg directory, ready for integration into your web project.

### Usage

The library's functionality is encapsulated within the hand module, where the main logic for calculating poker odds resides. While main.rs is present, the primary entry point to the library's functionality is through lib.rs.

Detailed examples and usage instructions will be added to this section soon.
 
### Contributing

This is my first project written in Rust, and I am eager to learn and improve. Contributions, whether in the form of feedback, bug reports, or code contributions, are warmly welcomed. Please feel free to open an issue or submit a pull request.

