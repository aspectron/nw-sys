use wasm_bindgen::prelude::*;
use js_sys::{Object, Function};
use crate::options::OptionsExt;

#[wasm_bindgen]
extern "C" {
    ///
    /// # Synopsis
    /// ```
    /// new Shortcut(&nw::shortcut::Option::new().key("Ctrl+Shift+A"))
    /// 
    /// //Register global desktop shortcut, which can work without focus.
    /// nw::App::register_global_hot_key(&shortcut);
    /// ```
    /// 
    /// Event:active
    /// Event:failed
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
    /// let shortcut = nw::Shortcut::new(&nw::shortcut::Options::new().key("Ctrl+Shift+A"));
    /// ```
    ///
    /// Create new Shortcut
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#new-shortcutoption)
    ///
    pub fn new(options:&Options) -> Shortcut;
    
    // shortcut.active
    // Get or set the active callback of a Shortcut. It will be called when user presses the shortcut.
    
    // shortcut.failed
    // *Get or set the failed callback of a Shortcut. It will be called when application passes an invalid key , or failed to register the key.
    
    // Event:active
    // Same as shortcut.active
    
    // Event:failed
    // Same as shortcut.failed

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
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutkey)
    pub fn key(self, key:&str)->Self{
        self.set("key", JsValue::from(key))
    }

    /// Set the active callback of a Shortcut.
    /// It will be called when user presses the shortcut.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutactive)
    pub fn active(self, callback:&Function)->Self{
        self.set("active", JsValue::from(callback))
    }

    /// Set the failed callback of a Shortcut.
    /// It will be called when application passes an invalid key,
    /// or failed to register the key.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Shortcut/#shortcutfailed)
    pub fn failed(self, callback:&Function)->Self{
        self.set("failed", JsValue::from(callback))
    }

    
}
