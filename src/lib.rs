//!
//! # Overview
//! 
//! [`nw-sys`] provides [`wasm_bindgen`] bindings for [Node Webkit JavaScript APIs](https://nwjs.readthedocs.io/en/latest/) 
//! offered by the [NWJS](https://nwjs.io) project, allowing development of 
//! interactive desktop applications based on Node Webkit in Rust using WASM.
//! 
//! `nw-sys` provides bindings for all Node Webkit subsystems offering:
//! - [`app`] application control and information access
//! - [`clipboard`] system clipboard access
//! - [`menu`] creation of application and tray menus
//! - [`screen`] access to system Display information and layout 
//! - [`shell`] external application execution, file and URL opening
//! - [`shortcut`] creation of application keyboard shortcuts 
//! - [`tray`] creation and installation of system tray menus
//! - [`window`] creation and control of application windows
//! 
//! This crate also implements a variety of helper structures to simplify 
//! access to various data using Rust.
//! 
//! Interactive installers for applications created using this crate targeting Windows, 
//! MacOS and Linux operating systems can be built using [`cargo-nw`](https://crates.io/crates/cargo-nw) 
//! packaging tool.
//!
//! An example application demonstrating use of these APIs can be 
//! found here:  [https://github.com/aspectron/nw-sys-example](https://github.com/aspectron/nw-sys-example)
//! 

pub mod error;
pub mod result;
pub mod options;

mod nw;
pub use nw::try_nw;
pub use nw::is_nw;

pub mod app;
//pub use app::App;

pub mod clipboard;
pub use clipboard::Clipboard;

pub mod menu;
pub use menu::Menu;

pub mod menu_item;
pub use menu_item::MenuItem;

pub mod screen;
pub use screen::Screen;

pub mod shell;
pub use shell::Shell;

pub mod shortcut;
pub use shortcut::Shortcut;

pub mod tray;
pub use tray::Tray;

pub mod window;
pub use window::Window;

pub mod prelude;
pub mod utils;

