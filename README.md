# Exclusive

[![travis build status](https://travis-ci.com/nvzqz/exclusive-rs.svg?branch=master)](https://travis-ci.com/nvzqz/exclusive-rs)
[![crates.io](https://img.shields.io/crates/v/exclusive.svg)
![downloads](https://img.shields.io/crates/d/exclusive.svg)](https://crates.io/crates/exclusive)
[![API docs](https://docs.rs/exclusive/badge.svg)](https://docs.rs/exclusive)
![rustc ^1.30.0](https://img.shields.io/badge/rustc-^1.30.0-blue.svg)

A procedural macro for wrapping code in a collision-free context.

## Installation

This crate is available [on crates.io](https://crates.io/crates/exclusive) and
can be used by adding the following to your project's
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[dependencies]
exclusive = "0.1.0"
```

and this to your crate root (`main.rs` or `lib.rs`):

```rust
#[macro_use]
extern crate exclusive;
```

## Usage

The [`exclusive!`] macro allows for placing blocks of code in a context that
won't collide with other calls to the macro in the same namespace:

```rust
exclusive! {
    let x = 20;
    let y = 30;
}

exclusive! {
    // This code doesn't actually run
    println!("Hello, world!");
}
```

# Purpose

This project was made to be used by
[`static_assertions`](https://github.com/nvzqz/static-assertions-rs) to prevent
different assertions from colliding with the same identifier.

## Changes

See [`CHANGELOG.md`](https://github.com/nvzqz/exclusive-rs/blob/master/CHANGELOG.md)
for a complete list of what has changed from one version to another.

## License

This project is released under either:

- [MIT License](https://github.com/nvzqz/exclusive-rs/blob/master/LICENSE-MIT)

- [Apache License (Version 2.0)](https://github.com/nvzqz/exclusive-rs/blob/master/LICENSE-APACHE)

at your choosing.

[`exclusive!`]: https://docs.rs/exclusive/*/exclusive/macro.exclusive.html
