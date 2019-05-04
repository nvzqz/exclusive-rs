//! Create exclusive contexts around your code that should not collide with
//! other calls to `exclusive!`.
//!
//! The `exclusive!` macro works by making a "unique" ID for each call site and
//! using that ID to create a unique `const` declaration.
//!
//! # Purpose
//!
//! This project was made to be used by
//! [`static_assertions`](https://crates.io/crates/static_assertions) to prevent
//! different assertions from colliding with the same identifier.
//!
//! # Usage
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! exclusive = "0.1"
//! ```
//!
//! and this to your crate root (`main.rs` or `lib.rs`):
//!
//! ```
//! #[macro_use]
//! extern crate exclusive;
//! # fn main() {}
//! ```
//!
//! Then, when calling the macro, you can place any code you want inside it!
//!
//! ```
#![cfg_attr(feature = "nightly", doc = "# #![feature(underscore_const_names)]")]
//! # #[macro_use] extern crate exclusive;
//! # fn main() {}
//! exclusive! {
//!     let x = 20;
//!     let y = 30;
//! }
//!
//! exclusive! {
//!     // This code doesn't actually run
//!     println!("Hello, world!");
//! }
//! ```
//!
//! With the `nightly` feature enabled, a `const` the identifier `_` is used
//! internally instead. This simply avoids having to generate a new identifier
//! each time.
//!
#![cfg_attr(feature = "nightly", doc = "```")]
#![cfg_attr(not(feature = "nightly"), doc = "```ignore")]
//! #![feature(underscore_const_names)]
//! # #[macro_use] extern crate exclusive;
//! # fn main() {}
//!
//! exclusive! {
//!     // Do stuff here
//! }
//!
//! exclusive! {
//!     // Do other stuff here
//! }
//! ```
//!
//! [crate]: https://crates.io/crates/exclusive
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html

extern crate proc_macro;

use proc_macro::*;

// Converts `span` into a unique identifier suitable for naming a `const`
#[cfg(not(feature = "nightly"))]
fn span_id(span: &Span) -> String {
    fn bytes_of<T>(val: &T) -> &[u8] {
        let ptr = val as *const T as *const u8;
        let len = std::mem::size_of::<T>();
        unsafe { std::slice::from_raw_parts(ptr, len) }
    }

    let table: &[u8; 16] = b"0123456789ABCDEF";
    let bytes = bytes_of(span);

    let mut id = b"_EXCLUSIVE_".to_vec();
    id.reserve_exact(bytes.len() * 2);

    for &byte in bytes {
        let byte = byte as usize;
        id.push(table[byte % 16]);
        id.push(table[(byte >> 4) % 16]);
    }

    unsafe { String::from_utf8_unchecked(id) }
}

/// Creates an exclusive context. 😎
///
/// # Examples
///
/// The code inside gets wrapped into a context that's guaranteed to not collide
/// with other calls of this macro in the same namespace:
///
/// ```
#[cfg_attr(feature = "nightly", doc = "# #![feature(underscore_const_names)]")]
/// # #[macro_use] extern crate exclusive;
/// # fn main() {}
/// exclusive! {
///     let x = 20;
///     let y = 30;
/// }
///
/// exclusive! {
///     // This code doesn't actually run
///     println!("Hello, world!");
/// }
/// ```
#[proc_macro]
pub fn exclusive(input: TokenStream) -> TokenStream {
    let call_site = Span::call_site();

    #[cfg(feature = "nightly")]
    let const_name = "_";

    #[cfg(not(feature = "nightly"))]
    let const_name = span_id(&call_site);

    // const $const_name: fn() -> () = || { $input };
    vec![
        TokenTree::Ident(Ident::new("const", call_site)),
        TokenTree::Ident(Ident::new(&*const_name, call_site)),
        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        TokenTree::Ident(Ident::new("fn", call_site)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        TokenTree::Punct(Punct::new('-', Spacing::Joint)),
        TokenTree::Punct(Punct::new('>', Spacing::Joint)),
        TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
        TokenTree::Punct(Punct::new('|', Spacing::Joint)),
        TokenTree::Punct(Punct::new('|', Spacing::Joint)),
        TokenTree::Group(Group::new(Delimiter::Brace, input)),
        TokenTree::Punct(Punct::new(';', Spacing::Alone)),
    ].into_iter().collect()
}
