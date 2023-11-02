<h1 align="center">📜 floc</h1>
<p align="center">Count the lines in your codebase</h1>

# Installation

## Via `cargo install`

You can install `floc` by installing the crates on [crates.io](https://crates.io).

Requirements:
- [Cargo](https://rust-lang.org) 1.73.0 or higher

```sh
cargo install floc
```

`floc` should now be installed in the Cargo `bin` directory. Nice! 🎉

## Via source code

You can install `floc` by building the project manually.

Requirements:
- [Cargo](https://rust-lang.org) 1.73.0 or higher

```sh
git clone https://github.com/interrrp/floc
cd floc
cargo install
```

`floc` should now be installed in the Cargo `bin` directory. Nice! 🎉

# Usage

By default, `floc` looks for a `src` directory with Rust (`.rs`) files:

```sh
floc  # Search for .rs files in the "src" directory
```

You can change the directory and extensions:

```sh
floc code               # Search in the "code" directory
floc app -e py -e html  # Search in the "app" directory for .py and .html files
```

# License

`floc` is licensed under the [MIT license](./LICENSE).
