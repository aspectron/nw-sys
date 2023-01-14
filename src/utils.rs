//!
//! Helper utilities for the browser Window, Document and DOM element access
//!

use web_sys::{Document, HtmlElement, Window};

/// Return Document element
pub fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("unable to get `document` node");
    document
}

/// Return Body element
pub fn body(win: Option<Window>) -> HtmlElement {
    let window = win.unwrap_or(web_sys::window().expect("no global `window` exists"));
    let document = window.document().expect("unable to get `document` node");
    document.body().expect("unable to get `document.body` node")
}

/// Return Window object
pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}
