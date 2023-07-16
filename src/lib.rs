//!
//! [<img alt="github" src="https://img.shields.io/badge/github-aspectron/nw--sys-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/aspectron/nw-sys)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/nw-sys.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/nw-sys)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-nw--sys-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/nw-sys)
//! <img alt="license" src="https://img.shields.io/crates/l/nw-sys.svg?maxAge=2592000&color=6ac&style=for-the-badge&logo=opensourceinitiative&logoColor=fff" height="20">
//!
//! # Overview
//!
//! [`nw-sys`](self) provides [`wasm_bindgen`] bindings for
//! [Node Webkit JavaScript APIs](https://nwjs.readthedocs.io/en/latest/)
//! offered by the [NWJS](https://nwjs.io) project, allowing development of
//! interactive desktop applications based on Node Webkit in Rust using WASM.
//!
//! Various helper structures that simplify this API and provide the Rust builder
//! pattern are available via the [`workflow-nw`](https://crates.io/crates/workflow-nw) crate.
//!
//! Interactive installers for applications created using this crate targeting Windows,
//! MacOS and Linux operating systems can be built using
//! [`cargo-nw`](https://crates.io/crates/cargo-nw) packaging tool.
//!
//! An example application demonstrating use of these APIs can be
//! found here:  [https://github.com/aspectron/nw-sys-example](https://github.com/aspectron/nw-sys-example)
//!
//! ---
//! This framework is maintained by [ASPECTRON](https://aspectron.org/). If you find this crate useful,
//! please consider supporting us. For more information, please visit [https://aspectron.org](https://aspectron.org/).
//!
//! ---
//!

pub mod error;
pub mod options;
pub mod result;

mod nw;
pub use nw::is_nw;
pub use nw::try_nw;
pub use nw::Nw;

pub mod global;

pub mod app;

pub mod clipboard;
#[doc(inline)]
pub use clipboard::Clipboard;

pub mod menu;
#[doc(inline)]
pub use menu::Menu;

pub mod menu_item;
#[doc(inline)]
pub use menu_item::MenuItem;

pub mod screen;

pub mod shell;

pub mod shortcut;
#[doc(inline)]
pub use shortcut::Shortcut;

pub mod tray;
#[doc(inline)]
pub use tray::Tray;

pub mod window;
#[doc(inline)]
pub use window::Window;

pub mod prelude;
pub mod utils;

mod chrome;
#[doc(inline)]
pub use chrome::notifications;
