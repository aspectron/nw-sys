use wasm_bindgen::prelude::*;
use js_sys::Object;
//use crate::nw::Nw;


#[wasm_bindgen]
extern "C" {

    //     Window
    // Window
    // Synopsis
    // Window.get([window_object])
    // Window.getAll(callback)
    // Window.open(url, [options], [callback])
    // win.window
    // win.x
    // win.y
    // win.width
    // win.height
    // win.title
    // win.menu
    // win.isAlwaysOnTop
    // win.isFullscreen
    // win.isTransparent
    // win.isKioskMode
    // win.zoomLevel
    // win.cookies.*
    // win.moveTo(x, y)
    // win.moveBy(x, y)
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
    /// Get the current window
    /// ```
    /// let win = Nw::get_window_ns()::get();
    /// // Listen to the minimize event
    /// win.on('minimize', function() {
    ///   console.log('Window is minimized');
    /// });
    ///
    /// // Minimize the window
    /// win.minimize();
    ///
    /// // Unlisten the minimize event
    /// win.removeAllListeners('minimize');
    ///
    /// // Create a new window and get it
    /// nw.Window.open('https://github.com', {}, function(new_win) {
    ///   // And listen to new window's focus event
    ///   new_win.on('focus', function() {
    ///     console.log('New window is focused');
    ///   });
    ///
    /// });
    /// ```
    #[wasm_bindgen(js_namespace=nw, js_name = Window)]
    #[derive(Debug, Clone)]
    pub type NWWindow;

    /// Window 
    ///
    /// [NWJS documentation](https://docs.nwjs.io/en/latest/References/Window/)
    //#[wasm_bindgen()]
    //#[derive(Debug, Clone)]
    //pub static WINDOW:JsValue;

    #[wasm_bindgen(static_method_of=Window, js_namespace=["nw"], js_name = get)]
    #[doc = "Get active window."]
    #[doc = ""]
    #[doc = "[NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowgetwindow_object)"]
    #[doc = ""]
    pub fn get() -> NWWindow;

    #[wasm_bindgen(static_method_of=Window, js_namespace=["nw"], js_name = open)]
    #[doc = "Get active window."]
    #[doc = ""]
    #[doc = "[NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)"]
    #[doc = ""]
    pub fn open(url:&str);


    #[wasm_bindgen(extends = Object , js_name = WindowOptions)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type WindowOptions;

    #[wasm_bindgen(static_method_of=Window, js_namespace=["nw"], js_name = open)]
    #[doc = "Get active window."]
    #[doc = ""]
    #[doc = "[NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)"]
    #[doc = ""]
    pub fn open_with_options(url:&str, option:&WindowOptions);

    /*
    #[wasm_bindgen(js_namespace=["nw"], js_name = Window)]
    #[doc = "Getter for the `Nw` field of this object."]
    #[doc = ""]
    #[doc = "[NWJS Documentation](https://docs.nwjs.io/en/latest/)"]
    #[doc = ""]
    pub fn get_window_ns_impl() -> Window;
    */

    
}


impl WindowOptions{
    /// "Construct a new `WindowOptions`.
    ///
    pub fn new() -> Self {
        #[allow(unused_mut)]
        let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(Object::new());
        ret
    }


    pub fn set(self, key:&str, value:JsValue) -> Self {
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

    /// The default title of window created by NW.js, .
    /// it's very useful if you want to show your own title when the app is starting
    ///
    pub fn title(self, title: &str) -> Self {
        self.set("title", JsValue::from(title))
    }

    /// the initial inner width of the main window.
    ///
    pub fn width(self, width: u32) ->Self {
        self.set("width", JsValue::from(width))
    }

    /// the initial inner height of the main window.
    ///
    pub fn height(self, height: u32) -> Self {
        self.set("height", JsValue::from(height))
    }

}


impl std::fmt::Display for WindowOptions{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

pub struct Window{

}



/*
impl Nw {
    //pub fn get_window_ns()->Window{
    //    get_window_ns_impl()
    //}
}
*/