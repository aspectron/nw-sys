//!
//! Provides access to the global `nw` namespace and [`try_nw`] functions allowing
//! to detect if the application is running inside of Node Webkit or in the browser.
//!

use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Access to the global `nw` namespace.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/)
    ///
    #[wasm_bindgen (extends = Object, js_name = nw)]
    #[derive(Debug, Clone)]
    pub type Nw;

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

/// Getter for the global `nw` namespace object
///
///
/// ⧉ [NWJS Documentation]: https://docs.nwjs.io/en/latest/
pub fn try_nw() -> Result<Nw, JsValue> {
    let nw_opt = Function::new_no_args("return this.nw").call0(&JsValue::undefined());

    match nw_opt {
        Ok(value) => {
            if value.is_undefined() {
                error("NW not found");
                Err(value)
            } else {
                let nw_ns: Nw = value.into();
                Ok(nw_ns)
            }
        }
        Err(err) => {
            error(&format!("NW not found, error: {:?}", err));
            Err(err)
        }
    }
}

/// Helper to test whether the application is running under
/// Node Webkit or in a regular browser environment.
pub fn is_nw() -> bool {
    try_nw().is_ok()
}
