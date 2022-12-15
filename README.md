# `nw-sys`

Rust bindings for NWJS API using `wasm_bindgen`

NWJS API documentation https://nwjs.readthedocs.io/en/latest/

### Examples

The following example application demonstrates the use of this API:
https://github.com/aspectron/nw-sys-example

### Building

You can use [`cargo-nw`](https://crates.io/crates/cargo-nw) utility to build installation packages for NW applications for Windows, MacOS and Linux.

### Status

* `app` - partial
* `window` - partial (Pending: win.cookies)
* `menu` - full
* `nw` - full
* `shell` - full
* `tray` - full
* `clipboard` - TODO
* `screen` - TODO
* `shortcut` - TODO