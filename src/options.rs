use wasm_bindgen::prelude::*;
use js_sys::Object;

pub trait OptionsExt {
    /// "Construct a new `Options`.
    ///
    fn new() -> Self 
    where Self:wasm_bindgen::JsCast
    {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(Object::new());
        ret = ret.initialize();
        ret
    }

    fn initialize(self)->Self
    where Self:wasm_bindgen::JsCast
    {
        self
    }

    fn set(self, key:&str, value:JsValue) -> Self
    where Self:wasm_bindgen::JsCast
    {
        let r = ::js_sys::Reflect::set(
            self.as_ref(),
            &JsValue::from(key),
            &value,
        );
        debug_assert!(
            r.is_ok(),
            "setting properties should never fail on our dictionary objects"
        );
        let _ = r;
        self
    }
}
