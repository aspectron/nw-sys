# `nw-sys`

## `wasm_bindgen` bindings for NWJS API

[<img alt="github" src="https://img.shields.io/badge/github-aspectron/nw--sys-8da0cb?style=for-the-badge&labelColor=555555&color=8da0cb&logo=github" height="20">](https://github.com/aspectron/nw-sys)
[<img alt="crates.io" src="https://img.shields.io/crates/v/nw-sys.svg?maxAge=2592000&style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/nw-sys)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-nw--sys-56c2a5?maxAge=2592000&style=for-the-badge&logo=rust" height="20">](https://docs.rs/nw-sys)
<img alt="license" src="https://img.shields.io/crates/l/nw-sys.svg?maxAge=2592000&color=6ac&style=for-the-badge&logoColor=fff" height="20">


NWJS JS API documentation is available here: https://nwjs.readthedocs.io/en/latest/

## Bindings

`nw-sys` provides Rust bindings for all NWJS subsystems with the following modules offering:
- `app` application control and information access
- `clipboard` system clipboard access
- `menu` creation of application and tray menus
- `screen` access to system Display information and layout 
- `shell` external application execution, file and URL opening
- `shortcut` creation of application keyboard shortcuts 
- `tray` creation and installation of system tray menus
- `window` creation and control of application windows

A higher-level Rust API for NWJS, based on top of this crate, is also available via the [`workflow-nw`](https://crates.io/crates/workflow-nw) crate that is a part of the [`workflow-rs`](https://github.com/workflow-rs/workflow-rs) application development framework.

## Examples

You can find an example application using these APIs at https://github.com/aspectron/nw-sys-example

## Other Crates

Following crates can be used in conjunciton with `nw-sys`:
- [`wasm_bindgen`](https://crates.io/crates/wasm_bindgen)
- [`js-sys`](https://crates.io/crates/js-sys)
- [`web-sys`](https://crates.io/crates/web-sys)
- [`node-sys`](https://crates.io/crates/node-sys)
- [`workflow-nw`](https://crates.io/crates/workflow-nw)
- [`workflow-panic-hook`](https://crates.io/crates/workflow-panic-hook)
- [`workflow-rs`](https://github.com/workflow-rs/workflow-rs)
