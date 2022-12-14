use wasm_bindgen::prelude::*;
use js_sys::{Function, Object};
//use wasm_bindgen::JsCast;
use workflow_log::log_trace;

#[wasm_bindgen]
extern "C" {

    /*
    #[wasm_bindgen (extends = :: js_sys :: Object, js_name=Object)]
    pub type Global;

    #[wasm_bindgen(getter, catch, static_method_of = Global, js_class = globalThis, js_name = globalThis)]
    fn get_global_this() -> Result<Object, JsValue>;

    #[wasm_bindgen(getter, catch, static_method_of = Global, js_class = self, js_name = self)]
    fn get_self() -> Result<Object, JsValue>;

    #[wasm_bindgen(getter, catch, static_method_of = Global, js_class = window, js_name = window)]
    fn get_window() -> Result<Object, JsValue>;

    #[wasm_bindgen(getter, catch, static_method_of = Global, js_class = global, js_name = global)]
    fn get_global() -> Result<Object, JsValue>;
    */
    

    #[wasm_bindgen (extends = Object)]
    #[derive(Debug, Clone)]
    /// The `nw` namespace.
    ///
    /// [NWJS documentation](https://docs.nwjs.io/en/latest/)
    ///
    pub type NwObject;

    
    /*
    #[wasm_bindgen(structural, catch, getter, method, js_class = nw1, js_name = nw1)]
    #[doc = "Getter for the `Nw` field of this object."]
    #[doc = ""]
    #[doc = "[NWJS Documentation](https://docs.nwjs.io/en/latest/)"]
    #[doc = ""]
    pub fn get_nw(this:&Global) -> Result<NwObject, JsValue>;
    */
    
}
/*
fn get_global_object() -> Object {
    // The order is important: in Firefox Extension Content Scripts `globalThis`
    // is a Sandbox (not Window), so `globalThis` must be checked after `window`.
    let static_object = Global::get_self()
        .or_else(|_| Global::get_window())
        .or_else(|_| Global::get_global_this())
        .or_else(|_| Global::get_global());
    if let Ok(obj) = static_object {
        if !obj.is_undefined() {
            return obj;
        }
    }

    // According to StackOverflow you can access the global object via:
    //
    //      const global = Function('return this')();
    //
    // I think that's because the manufactured function isn't in "strict" mode.
    // It also turns out that non-strict functions will ignore `undefined`
    // values for `this` when using the `apply` function.
    //
    // As a result we use the equivalent of this snippet to get a handle to the
    // global object in a sort of roundabout way that should hopefully work in
    // all contexts like ESM, node, browsers, etc.
    let this = Function::new_no_args("return this")
        .call0(&JsValue::undefined())
        .ok();

    // Note that we avoid `unwrap()` on `call0` to avoid code size bloat, we
    // just handle the `Err` case as returning a different object.
    debug_assert!(this.is_some());
    match this {
        Some(this) => this.unchecked_into(),
        None => JsValue::undefined().unchecked_into(),
    }
}
*/


/// Getter for the `Nw` object
///
/// [NWJS Documentation]
///
/// [NWJS Documentation]: https://docs.nwjs.io/en/latest/
pub fn try_nw() -> Result<NwObject, JsValue> {

    //return Err(JsValue::undefined());
    
    let nw_opt = Function::new_no_args("return this.nw")
        .call0(&JsValue::undefined());

    match nw_opt{
        Ok(value)=>{
            if value.is_undefined(){
                log_trace!("nw not found");
                Err(value)
            }else{
                //log_trace!("nw_opt: {:?}", value);
                let nw_ns:NwObject = value.clone().into();
                Ok(nw_ns)
            }
        }
        Err(err)=>{
            log_trace!("nw not found, error: {:?}", err);
            Err(err)
        }
    }
    
    //get_global_object().dyn_into::<Global>().unwrap().get_nw()
}

pub fn is_nw()->bool{
    try_nw().is_ok()
}

//#[allow(non_snake_case)]
pub mod nw{
    pub use crate::nw::try_nw;
    pub use crate::nw::is_nw;
    pub use crate::window::Window;
    pub mod window{
        pub use crate::window::*;
    }
    pub use crate::menu::Menu;
    pub mod menu{
        pub use crate::menu::*;
    }
    pub use crate::menu_item::MenuItem;
    pub mod menu_item{
        pub use crate::menu_item::*;
    }
}
