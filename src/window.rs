use wasm_bindgen::prelude::*;
use js_sys::{Object, Function};
use crate::options::OptionsExt;
use crate::menu::Menu;

#[wasm_bindgen]
extern "C" {

    
    // win.window
    // win.menu
    // win.cookies.*
    // win.resizeTo(width, height)
    // win.setInnerWidth(width)
    // win.setInnerHeight(height)
    // win.resizeBy(width, height)
    // win.focus()
    // win.blur()
    // win.show([is_show])
    // win.hide()
    // win.close([force])
    // win.reload()
    // win.reloadDev()
    // win.reloadIgnoringCache()
    // win.maximize()
    // win.unmaximize()
    // win.minimize()
    // win.restore()
    // win.enterFullscreen()
    // win.leaveFullscreen()
    // win.toggleFullscreen()
    // win.enterKioskMode()
    // win.leaveKioskMode()
    // win.toggleKioskMode()
    // win.setTransparent(transparent)
    // win.setShadow(shadow) (Mac)
    // win.showDevTools([iframe], [callback])
    // win.closeDevTools()
    // win.getPrinters(callback)
    // win.isDevToolsOpen()
    // win.print(options)
    // win.setMaximumSize(width, height)
    // win.setMinimumSize(width, height)
    // win.setResizable(resizable)
    // win.setAlwaysOnTop(top)
    // win.setVisibleOnAllWorkspaces(visible) (Mac and Linux)
    // win.canSetVisibleOnAllWorkspaces() (Mac and Linux)
    // win.setPosition(position)
    // win.setShowInTaskbar(show)
    // win.requestAttention(attension)
    // win.capturePage(callback [, config ])
    // win.captureScreenshot(options [, callback])
    // win.setProgressBar(progress)
    // win.setBadgeLabel(label)
    // win.eval(frame, script)
    // win.evalNWBin(frame, path)
    // win.evalNWBinModule(frame, path, module_path)
    // win.removeAllListeners([eventName])
    // Event: close
    // Event: closed
    // Event: loading
    // Event: loaded
    // Event: document-start(frame)
    // Event: document-end(frame)
    // Event: focus
    // Event: blur
    // Event: minimize
    // Event: restore
    // Event: maximize
    // Event: move(x, y)
    // Event: resize(width, height)
    // Event: enter-fullscreen
    // Event: leave-fullscreen
    // Event: zoom
    // Event: capturepagedone
    // Event: devtools-opened(url)
    // Event: devtools-closed
    // Event: new-win-policy (frame, url, policy)
    // Event: navigation (frame, url, policy)
    // Window is a wrapper of the DOM’s topmost window object. It has extended operations and can receive various window events.

    // Every Window is an instance of the EventEmitter class, and you’re able to use Window.on(...) to respond to native window’s events.

    // Behavior Changed

    // There are some changes of Window since 0.13.0. Please see Migration Notes from 0.12 to 0.13.



    /// # Synopsis
    /// 
    /// ```
    /// use workflow_wasm::listener::Listener;
    /// 
    /// // Get the current window
    /// let win = nw::Window()::get();
    /// // Listen to the minimize event
    /// win.on("minimize", |_| {
    ///   log_info!("Window is minimized");
    /// });
    ///
    /// // Minimize the window
    /// win.minimize();
    ///
    /// // Unlisten the minimize event
    /// win.remove_all_listeners("minimize");
    ///
    /// // Create a new window and get it
    /// let options = nw::window::Options::new()
    ///     .title("Test window");
    /// 
    /// let listener = Listener::new(|new_win| {
    ///   // And listen to new window's focus event
    ///   new_win.on("focus", function() {
    ///     log_info!("New window is focused");
    ///   });
    /// });
    /// 
    /// nw::Window::open_with_options_and_callback(
    ///     "https://github.com",
    ///     &options,
    ///     listener.into_js()
    /// );
    /// 
    /// //save this listener somewhere otherwise it will leak memory 
    /// listener.forget();
    /// ```
    #[wasm_bindgen(js_namespace=nw, js_name = Window)]
    #[derive(Debug, Clone)]
    pub type Window;

    #[wasm_bindgen(method, getter, js_name = window)]
    /// Get the corresponding DOM window object of the native window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winwindow)
    ///
    pub fn window(this:&Window)->web_sys::Window;

    #[wasm_bindgen(method, getter, js_name = x)]
    /// Get left offset from window frame to screen.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winx)
    ///
    pub fn x(this:&Window)->i32;

    #[wasm_bindgen(method, setter, js_name = x)]
    /// Set left offset from window frame to screen.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winx)
    ///
    pub fn set_x(this:&Window, x:i32);

    #[wasm_bindgen(method, getter, js_name = y)]
    /// Get top offset from window frame to screen.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winy)
    ///
    pub fn y(this:&Window)->i32;

    #[wasm_bindgen(method, setter, js_name = y)]
    /// Set top offset from window frame to screen.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winy)
    ///
    pub fn set_y(this:&Window, y:i32);

    #[wasm_bindgen(method, getter, js_name = width)]
    /// Get window’s size, including the window’s frame.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winwidth)
    ///
    pub fn width(this:&Window)->u32;

    #[wasm_bindgen(method, setter, js_name = width)]
    /// Set window’s size, including the window’s frame.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winwidth)
    ///
    pub fn set_width(this:&Window, width:u32);

    #[wasm_bindgen(method, getter, js_name = height)]
    /// Get window’s size, including the window’s frame.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winheight)
    ///
    pub fn height(this:&Window)->u32;

    #[wasm_bindgen(method, setter, js_name = height)]
    /// Set window’s size, including the window’s frame.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winheight)
    ///
    pub fn set_height(this:&Window, height:u32);

    #[wasm_bindgen(method, getter, js_name = title)]
    /// Get window’s title.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wintitle)
    ///
    pub fn title(this:&Window)->String;

    #[wasm_bindgen(method, setter, js_name = title)]
    /// Set window’s title.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wintitle)
    ///
    pub fn set_title(this:&Window, title:&str);

    #[wasm_bindgen(method, getter, js_name = menu)]
    /// Get window’s menubar.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winmenu)
    ///
    pub fn menu(this:&Window)->Menu;

    #[wasm_bindgen(method, setter, js_name = menu)]
    /// Set window’s menubar.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winmenu)
    ///
    pub fn set_menu(this:&Window, menu:&Menu);

    #[wasm_bindgen(method, setter, js_name = menu)]
    /// Set window’s menubar = null.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winmenu)
    ///
    pub fn remove_menu_impl(this:&Window, menu:JsValue);

    #[wasm_bindgen(method, getter, js_name = isAlwaysOnTop)]
    /// Get whether the window is always on top of other windows.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winisalwaysontop)
    ///
    pub fn is_always_on_top(this:&Window)->bool;

    #[wasm_bindgen(method, getter, js_name = isFullscreen)]
    /// Get whether we’re in fullscreen mode.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winisfullscreen)
    ///
    pub fn is_fullscreen(this:&Window)->bool;

    #[wasm_bindgen(method, getter, js_name = isTransparent)]
    /// Get whether transparency is turned on
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winistransparent)
    ///
    pub fn is_transparent(this:&Window)->bool;

    #[wasm_bindgen(method, getter, js_name = isKioskMode)]
    /// Get whether we’re in kiosk mode.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winiskioskmode)
    ///
    pub fn is_kiosk_mode(this:&Window)->bool;

    #[wasm_bindgen(method, getter, js_name = zoomLevel)]
    /// Get the page zoom. 0 for normal size; positive value for zooming in; negative value for zooming out.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winzoomlevel)
    ///
    pub fn zoom_level(this:&Window)->i16;

    #[wasm_bindgen(method, setter, js_name = zoomLevel)]
    /// Set the page zoom. 0 for normal size; positive value for zooming in; negative value for zooming out.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winzoomlevel)
    ///
    pub fn set_zoom_level(this:&Window, zoom:i16);

    //TODO: Cookies

    #[wasm_bindgen(method, js_name = moveTo)]
    /// Moves a window’s left and top edge to the specified coordinates.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winmovetox-y)
    ///
    pub fn move_to(this:&Window, x:u32, y:u32);

    #[wasm_bindgen(method, js_name = moveBy)]
    /// Moves a window a specified number of pixels relative to its current coordinates.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winmovebyx-y)
    ///
    pub fn move_by(this:&Window, x:u32, y:u32);

    #[wasm_bindgen(static_method_of=Window, js_namespace=["nw"], js_name = get)]
    /// Get current window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowgetwindow_object)
    ///
    pub fn get() -> Window;

    #[wasm_bindgen(static_method_of=Window, js_namespace=["nw"], js_name = open)]
    /// Open new window
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    ///
    pub fn open(url:&str);

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Options;

    #[wasm_bindgen(static_method_of=Window, js_namespace=["nw"], js_name = open)]
    /// Open window with options
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    pub fn open_with_options(url:&str, option:&Options);

    #[wasm_bindgen(static_method_of=Window, js_namespace=["nw"], js_name = open)]
    /// Open window with options and callback.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    pub fn open_with_options_and_callback(url:&str, option:&Options, callback:&Function);

}


impl Window{
    /// Set window’s menubar = null.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winmenu)
    ///
    pub fn remove_menu(&self){
        self.remove_menu_impl(JsValue::null());
    }
}

impl OptionsExt for Options{}

impl Options{

    /// the id used to identify the window.
    /// This will be used to remember the size and position of the window
    /// and restore that geometry when a window with the same id is later opened.
    /// [See also the Chrome App documentation](https://developer.chrome.com/docs/extensions/reference/app_window/#property-CreateWindowOptions-id)
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    pub fn id(self, id: &str) ->Self {
        self.set("id", JsValue::from(id))
    }

    /// The default title of window created by NW.js, .
    /// it's very useful if you want to show your own title when the app is starting
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Manifest%20Format/#webkit-subfields)
    pub fn title(self, title: &str) -> Self {
        self.set("title", JsValue::from(title))
    }

    /// the initial inner width of the window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Manifest%20Format/#webkit-subfields)
    pub fn width(self, width: u32) ->Self {
        self.set("width", JsValue::from(width))
    }

    /// the initial inner height of the window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Manifest%20Format/#webkit-subfields)
    pub fn height(self, height: u32) -> Self {
        self.set("height", JsValue::from(height))
    }

    /// the initial left of the window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Manifest%20Format/#webkit-subfields)
    pub fn left(self, left: u32) ->Self {
        self.set("x", JsValue::from(left))
    }

    /// the initial top of the window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Manifest%20Format/#webkit-subfields)
    pub fn top(self, top: u32) ->Self {
        self.set("y", JsValue::from(top))
    }


    /// whether to open a new window in a separate render process.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    pub fn new_instance(self, value: bool) ->Self {
        self.set("new_instance", JsValue::from(value))
    }

    /// If true, the Node context and DOM context are merged in the new window’s process.
    /// Use only when new_instance is true.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    pub fn mixed_context(self, value: bool) ->Self {
        self.set("mixed_context", JsValue::from(value))
    }

    /// the script to be injected before any DOM is constructed and any script is run. 
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    pub fn inject_js_start(self, js: &str) ->Self {
        self.set("inject_js_start", JsValue::from(js))
    }

    /// the script to be injected after the document object is loaded, before onload event is fired.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    pub fn inject_js_end(self, js: &str) ->Self {
        self.set("inject_js_end", JsValue::from(js))
    }

    

}


impl std::fmt::Display for Options{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

