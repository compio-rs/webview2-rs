//! WebView2 Win32 bindings for Rust
//!
//! This crate provides type bindings to the WebView2 API,
//! as well as a reimplementation of the WebView2Loader in pure Rust.

mod bindings;
pub use bindings::*;

mod native;
pub use native::*;
