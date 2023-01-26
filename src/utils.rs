//!
//! Helper utilities for the browser Window, Document and DOM element access
//!

use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement, Window};

/// Return Document element
pub fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    window.document().expect("unable to get `document` node")
}

/// Return Body element
pub fn body(win: Option<Window>) -> HtmlElement {
    let window = win.unwrap_or_else(|| web_sys::window().expect("no global `window` exists"));
    let document = window.document().expect("unable to get `document` node");
    document.body().expect("unable to get `document.body` node")
}

/// Return Window object
pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

/// Obtain a `u64` value from an object property.
/// Returns successfully parsed value or 0.
pub fn try_get_u64_from_prop(jsv: &JsValue, prop: &str) -> Result<u64, JsValue> {
    let v = js_sys::Reflect::get(jsv, &JsValue::from(prop))?;
    Ok(v.as_f64().ok_or_else(|| {
        JsValue::from(format!(
            "try_get_u64(): error parsing property '{}' with value '{:?}'",
            prop, v
        ))
    })? as u64)
}

/// Obtain `f64` value from an object property.
pub fn try_get_f64_from_prop(jsv: &JsValue, prop: &str) -> Result<f64, JsValue> {
    let v = js_sys::Reflect::get(jsv, &JsValue::from(prop))?;
    v.as_f64().ok_or_else(|| {
        JsValue::from(format!(
            "try_get_f64(): error parsing property '{}' with value '{:?}'",
            prop, v
        ))
    })
}

/// Obtain a `bool` value from the object property `prop`
pub fn try_get_bool_from_prop(jsv: &JsValue, prop: &str) -> Result<bool, JsValue> {
    js_sys::Reflect::get(jsv, &JsValue::from(prop))?
        .as_bool()
        .ok_or_else(|| {
            JsValue::from(format!(
                "try_get_bool(): property {} is missing or not a boolean",
                prop
            ))
        })
}

/// Obtain a `JsValue` value from the object property `prop`
pub fn try_get_js_value(this_jsv: &JsValue, prop: &str) -> Result<JsValue, JsValue> {
    let v = js_sys::Reflect::get(this_jsv, &JsValue::from(prop))?;
    Ok(v)
}
