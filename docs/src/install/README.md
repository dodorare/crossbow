# Installation

## Rust Setup

All `crossbow` crates are written in Rust. This means that before we begin, we need to set up our Rust development environment.

### Installing Rust

Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).

Once this is done, you should have the `rustc` compiler and the `cargo` build system installed in your path.

### Rust Learning Resources

The goal of this book is to learn `crossbow`, so it won't serve as a full Rust education. If you would like to learn more about the Rust language, check out the following resources:

- [The Rust Book](https://doc.rust-lang.org/book/): the best place to learn Rust from scratch.
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/): learn Rust by working through live coding examples.

## Crossbundle Setup

To begin with `crossbow` - you will need to install `crossbundle` first. You can do it simply by running the following command:

```sh
cargo install --git=https://github.com/dodorare/crossbow crossbundle
```

## Next steps

To complete the installation of all requirements, read the documentation corresponding to your platform:

- [Android on Windows](android-windows.md)
- [Android on Linux](android-linux.md)
- [Android on macOS](android-macos.md)
- [Android with Docker](docker.md)
- [iOS on macOS](ios-macos.md)
