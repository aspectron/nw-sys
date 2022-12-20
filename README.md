# `nw-sys`

Rust bindings for NWJS API using `wasm_bindgen`

NWJS API documentation https://nwjs.readthedocs.io/en/latest/

## Examples

The following example application demonstrates the use of this API:
https://github.com/aspectron/nw-sys-example

## Building

You can use [`cargo-nw`](https://crates.io/crates/cargo-nw) tool to build interactive Windows, MacOS and Linux installers for Node Webkit applications.

## Bindings

`nw-sys` provides bindings for all Node Webkit subsystems with following modules offering:
- `app` application control and information access
- `clipboard` system clipboard access
- `menu` creation of application and tray menus
- `screen` access to system Display information and layout 
- `shell` external application execution, file and URL opening
- `shortcut` creation of application keyboard shortcuts 
- `tray` creation and installation of system tray menus
- `window` creation and control of application windows

## Examples

You can find an example application using these APIs at https://github.com/aspectron/nw-sys-example

## Other Crates

Following crates can be used in conjunciton with `nw-sys`:
- [`wasm_bindgen`](https://crates.io/crates/wasm_bindgen)
- [`js-sys`](https://crates.io/crates/js-sys)
- [`web-sys`](https://crates.io/crates/web-sys)
- [`node-sys`](https://crates.io/crates/node-sys)
- [`node-process`](https://crates.io/crates/node-process)
- [`workflow-panic-hook`](https://crates.io/crates/workflow-panic-hook)
- [`workflow-rs`](https://github.com/workflow-rs/workflow-rs)
