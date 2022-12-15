use wasm_bindgen::prelude::*;
use js_sys::Object;


#[wasm_bindgen]
extern "C" {    
    /// # Synopsis
    /// ```
    /// // Open URL with default browser.
    /// nw::Shell::open_external("https://github.com/nwjs/nw.js");
    /// ```
    #[wasm_bindgen(extends = Object)]
    pub type Shell;

    #[wasm_bindgen(static_method_of=Shell, js_namespace=nw, js_name = openExternal)]
    /// Open the given external URI in the desktop’s default manner.
    /// 
    /// For example, mailto: URLs in the default mail user agent.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shell/#shellopenexternaluri)
    /// 
    pub fn open_external(uri:&str);
    
    #[wasm_bindgen(static_method_of=Shell, js_namespace=nw, js_name = openItem)]
    /// Open the given `file_path` in the desktop’s default manner.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shell/#shellopenitemfile_path)
    /// 
    pub fn open_item(file_path:&str);

    #[wasm_bindgen(static_method_of=Shell, js_namespace=nw, js_name = showItemInFolder)]
    /// Show the given `file_path` in the parent folder with file manager.
    /// 
    /// If possible, select the file.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shell/#shellshowiteminfolderfile_path)
    /// 
    pub fn show_item_in_folder(file_path:&str);
}
