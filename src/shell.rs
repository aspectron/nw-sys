//! 
//! Access to the system shell. Allows running external programs, open Shell 
//! (File Explorer / Finder) windows at different locations as well as 
//! launching external applications based on file type associations.
//! 
//!  # Synopsis
//! ```
//! // Open URL with default browser.
//! nw_sys::shell::open_external("https://github.com/nwjs/nw.js");
//! 
//! // Open a text file with default text editor.
//! nw_sys::shell::open_item("/absolute/path/to/file.txt");
//! 
//! // Show a file in parent folder with file manager.
//! nw_sys::shell::show_item_in_folder("/absolute/path/to/file.txt");
//! ```
//! 

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {    
    
    #[wasm_bindgen(js_namespace=["nw", "Shell"], js_name = openExternal)]
    /// Open the given external URI in the desktop’s default manner.
    /// 
    /// Shell (Explorer / Finder) interface. For usage example please refer to [nw_sys::shell](self)
    /// 
    /// For example, mailto: URLs in the default mail user agent.
    /// 
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shell/#shellopenexternaluri)
    /// 
    pub fn open_external(uri:&str);
    
    #[wasm_bindgen(js_namespace=["nw", "Shell"], js_name = openItem)]
    /// Open the given `file_path` in the desktop’s default manner.
    /// 
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shell/#shellopenitemfile_path)
    /// 
    pub fn open_item(file_path:&str);

    #[wasm_bindgen(js_namespace=["nw", "Shell"], js_name = showItemInFolder)]
    /// Show the given `file_path` in the parent folder with file manager.
    /// 
    /// If possible, select the file.
    /// 
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shell/#shellshowiteminfolderfile_path)
    /// 
    pub fn show_item_in_folder(file_path:&str);
}
