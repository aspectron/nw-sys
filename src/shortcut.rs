//! 
//! Register keyboard shurtcuts that will be received by your application.
//! 
//! # Synopsis
//! ```
//! use workflow_wasm::prelude::*;
//! 
//!  //Create a shortcut with |option|.
//! let shortcut = nw_sys::Shortcut::new(&nw_sys::shortcut::Options::new().key("Ctrl+Shift+A"));
//! 
//! // If register |shortcut| successfully and user struck "Ctrl+Shift+A", |shortcut|
//! // will get an "active" event.
//! 
//! // You can also add callback to shortcut's active and failed event.   
//! let callback = callback!(||{
//!     log_info!("Global desktop keyboard shortcut: 'Ctrl+Shift+A' active.");
//! });
//! shortcut.on_active(callback.as_ref());
//! 
//! // Register global desktop shortcut, which can work without focus.
//! nw_sys::app::register_global_hot_key(&shortcut);
//! 
//! // Unregister the global desktop shortcut.
//! nw_sys::app::unregister_global_hot_key(&shortcut);
//! 
//! //save callback
//! app.push_callback(callback)?;
//! 
//! ```
//! 

use wasm_bindgen::prelude::*;
use js_sys::{Object, Function};
use crate::options::OptionsExt;

#[wasm_bindgen]
extern "C" {
    ///
    /// Interface for registering keyboard shortcuts. For usage example please refer to [nw_sys::shortcut](self)
    ///
    /// Shortcut represents a global keyboard shortcut, 
    /// also known as system-wide hotkey. If registered successfully, 
    /// it works even if your app does not have focus.
    /// 
    /// Shortcut inherited from EventEmitter. 
    /// Every time the user presses the registered shortcut, 
    /// your app will receive an active event of the shortcut object.
    /// 
    #[wasm_bindgen(js_namespace=nw, js_name = Tray)]
    #[derive(Debug, Clone)]
    pub type Shortcut;

    
    #[wasm_bindgen(constructor, js_namespace=["nw"])]
    /// # Synopsis
    /// 
    /// ```
    /// //Create a Shortcut
    /// let shortcut = nw_sys::shortcut::new(&nw_sys::shortcut::Options::new().key("Ctrl+Shift+A"));
    /// ```
    ///
    /// Create new Shortcut
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#new-shortcutoption)
    ///
    pub fn new(options:&Options) -> Shortcut;
    
    #[wasm_bindgen(setter, method, js_namespace=["nw"], js_name=active)]
    /// Set the active callback of a Shortcut.
    /// It will be called when user presses the shortcut.
    /// 
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutactive)
    ///
    pub fn on_active(this:&Shortcut, callback:&Function);

    #[wasm_bindgen(setter, method, js_namespace=["nw"], js_name=failed)]
    /// Set the `failed` callback of a Shortcut.
    /// It will be called when application passes an invalid key ,
    /// or failed to register the key.
    /// 
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutactive)
    ///
    pub fn on_failed(this:&Shortcut, callback:&Function);

    // shortcut.failed
    // *Get or set the failed callback of a Shortcut. It will be called when application passes an invalid key , or failed to register the key.

    /// Shortcut Options
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Options;
}

impl OptionsExt for Options{}

impl Options{
    /// Set the `key` of a `Shortcut`.
    /// It is a string to specify the shortcut key, like "Ctrl+Alt+A".
    /// The key is consisted of zero or more modifiers and a key on your keyboard.
    /// Only one key code is supported. Key code is case insensitive.
    /// 
    /// ### List of supported modifiers:
    /// 
    /// - Ctrl
    /// - Alt
    /// - Shift
    /// - Command: Command modifier maps to Apple key (⌘) on Mac, 
    /// and maps to the Windows key on Windows and Linux.
    /// 
    /// ### List of supported keys:
    /// 
    /// - Alphabet: `A`-`Z`
    /// - Digits: `0`-`9`
    /// - Function Keys: `F1`-`F24`
    /// - Home / End / PageUp / PageDown / Insert / Delete
    /// - Up / Down / Left / Right
    /// - MediaNextTrack / MediaPlayPause / MediaPrevTrack / MediaStop
    /// - Comma or `,`
    /// - Period or `.`
    /// - Tab or `\t`
    /// - Backquote or `` ` ``
    /// - Enter or `\n`
    /// - Minus or `-`
    /// - Equal or `=`
    /// - Backslash or `\`
    /// - Semicolon or `;`
    /// - Quote or `'`
    /// - BracketLeft or `[`
    /// - BracketRight or `]`
    /// - Escape
    /// 
    /// 
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutkey)
    pub fn key(self, key:&str)->Self{
        self.set("key", JsValue::from(key))
    }

    /// Set the active callback of a Shortcut.
    /// It will be called when user presses the shortcut.
    /// 
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutactive)
    pub fn active(self, callback:&Function)->Self{
        self.set("active", JsValue::from(callback))
    }

    /// Set the failed callback of a Shortcut.
    /// It will be called when application passes an invalid key,
    /// or failed to register the key.
    /// 
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutfailed)
    pub fn failed(self, callback:&Function)->Self{
        self.set("failed", JsValue::from(callback))
    }

    
}
