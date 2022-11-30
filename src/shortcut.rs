use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    // Shortcut
    // Shortcut
    // Synopsis
    // new Shortcut(option)
    // shortcut.key
    // shortcut.active
    // shortcut.failed
    // Event:active
    // Event:failed
    // Shortcut represents a global keyboard shortcut, also known as system-wide hotkey. If registered successfully, it works even if your app does not have focus.
    
    // Shortcut inherited from EventEmitter. Every time the user presses the registered shortcut, your app will receive an active event of the shortcut object.
    
    // Synopsis
    // var option = {
    //   key : "Ctrl+Shift+A",
    //   active : function() {
    //     console.log("Global desktop keyboard shortcut: " + this.key + " active."); 
    //   },
    //   failed : function(msg) {
    //     // :(, fail to register the |key| or couldn't parse the |key|.
    //     console.log(msg);
    //   }
    // };
    
    // // Create a shortcut with |option|.
    // var shortcut = new nw.Shortcut(option);
    
    // // Register global desktop shortcut, which can work without focus.
    // nw.App.registerGlobalHotKey(shortcut);
    
    // // If register |shortcut| successfully and user struck "Ctrl+Shift+A", |shortcut|
    // // will get an "active" event.
    
    // // You can also add listener to shortcut's active and failed event.
    // shortcut.on('active', function() {
    //   console.log("Global desktop keyboard shortcut: " + this.key + " active."); 
    // });
    
    // shortcut.on('failed', function(msg) {
    //   console.log(msg);
    // });
    
    // // Unregister the global desktop shortcut.
    // nw.App.unregisterGlobalHotKey(shortcut);
    // new Shortcut(option)
    // option {Object}
    // key {String} key combinations of the shortcut, such as "ctrl+shift+a". See shortcut.key property for details.
    // active {Function} Optional a callback when the hotkey is triggered. See shortcut.active property for details.
    // failed {Function} Optional a callback when failed to register the hotkey. See shortcut.failed property for details.
    // Create new Shortcut, option is an object contains initial settings for the Shortcut.
    
    // shortcut.key
    // Get the key of a Shortcut. It is a string to specify the shortcut key, like "Ctrl+Alt+A". The key is consisted of zero or more modifiers and a key on your keyboard. Only one key code is supported. Key code is case insensitive.
    
    // List of supported modifiers:
    
    // Ctrl
    // Alt
    // Shift
    // Command: Command modifier maps to Apple key (⌘) on Mac, and maps to the Windows key on Windows and Linux.
    // List of supported keys:
    
    // Alphabet: A-Z
    // Digits: 0-9
    // Function Keys: F1-F24
    // Comma
    // Period
    // Tab
    // Home / End / PageUp / PageDown / Insert / Delete
    // Up / Down / Left / Right
    // MediaNextTrack / MediaPlayPause / MediaPrevTrack / MediaStop
    // Comma or ,
    // Period or .
    // Tab or \t
    // Backquote or `
    // Enter or \n
    // Minus or -
    // Equal or =
    // Backslash or \
    // Semicolon or ;
    // Quote or '
    // BracketLeft or [
    // BracketRight or ]
    // Escape
    // DOM Level 3 W3C KeyboardEvent Code Values
    // Single Key without Modifiers
    
    // The API App.registerGlobalHotKey() can support applications intercepting a single key (like { key: "A"}). But users will not be able to use “A” normally any more until the app unregisters it. However, the API doesn’t limit this usage, and it would be useful if the applications wants to listen Media Keys.
    // Only use zero modifier when you are knowing what your are doing.
    
    // shortcut.active
    // Get or set the active callback of a Shortcut. It will be called when user presses the shortcut.
    
    // shortcut.failed
    // *Get or set the failed callback of a Shortcut. It will be called when application passes an invalid key , or failed to register the key.
    
    // Event:active
    // Same as shortcut.active
    
    // Event:failed
    // Same as shortcut.failed

}
