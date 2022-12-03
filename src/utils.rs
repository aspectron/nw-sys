use web_sys::{
    Window,
    Document,
    HtmlElement
};

pub fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("unable to get `document` node");
    document
}

pub fn body(win:Option<Window>) -> HtmlElement {
    let window = win.unwrap_or(web_sys::window().expect("no global `window` exists"));
    let document = window.document().expect("unable to get `document` node");
    document.body().expect("unable to get `document.body` node")
}


pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}
