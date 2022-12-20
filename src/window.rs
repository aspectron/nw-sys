use wasm_bindgen::prelude::*;
use web_sys::HtmlIFrameElement;
use js_sys::{Object, Function, Promise, ArrayBuffer};
use crate::options::OptionsExt;
use crate::menu::Menu;

#[wasm_bindgen]
extern "C" {
    // TODO: win.cookies.*

    /// # Synopsis
    /// 
    /// ```rust
    /// use workflow_wasm::listener::Listener;
    /// 
    /// // Get the current window
    /// let win = nw::Window::get();
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
    ///   let focus_listener = Listener::new(||{
    ///     log_info!("New window is focused");
    ///   });
    ///   new_win.on("focus", focus_listener.into_js());
    /// });
    /// 
    /// nw::Window::open_with_options_and_callback(
    ///     "https://github.com",
    ///     &options,
    ///     listener.into_js()
    /// );
    /// 
    /// //save this `listener` somewhere
    /// 
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


    #[wasm_bindgen(method, js_name = resizeTo)]
    /// Resizes a window to the specified width and height.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winresizetowidth-height)
    ///
    pub fn resize_to(this:&Window, width:u32, height:u32);

    #[wasm_bindgen(method, js_name = setInnerWidth)]
    /// Set the inner width of the window
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetinnerwidthwidth)
    ///
    pub fn set_inner_width(this:&Window, width:u32);

    #[wasm_bindgen(method, js_name = setInnerHeight)]
    /// Set the inner height of the window
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetinnerheightheight)
    ///
    pub fn set_inner_height(this:&Window, height:u32);

    #[wasm_bindgen(method, js_name = resizeBy)]
    /// Resizes a window by the specified amount.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winresizebywidth-height)
    ///
    pub fn resize_by(this:&Window, width:u32, height:u32);

    #[wasm_bindgen(method, js_name = focus)]
    /// Focus on the window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winfocus)
    ///
    pub fn focus(this:&Window);

    #[wasm_bindgen(method, js_name = blur)]
    /// Move focus away. Usually it will move focus to other windows of your app,
    /// since on some platforms there is no concept of blur.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winblur)
    ///
    pub fn blur(this:&Window);

    #[wasm_bindgen(method, js_name = show)]
    /// Show the window if it’s not shown.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winshowis_show)
    ///
    pub fn show(this:&Window);

    #[wasm_bindgen(method, js_name = show)]
    /// Show/Hide the window
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winshowis_show)
    ///
    pub fn set_show(this:&Window, is_show:bool);

    #[wasm_bindgen(method, js_name = hide)]
    /// Hide the window. User will not be able to find the window once it’s hidden.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winhide)
    ///
    pub fn hide(this:&Window);

    #[wasm_bindgen(method, js_name = close)]
    /// Closes the current window.
    /// You can prevent the closing by listening to the close event.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincloseforce)
    ///
    pub fn close(this:&Window);

    #[wasm_bindgen(method, js_name = close)]
    /// Closes the current window.
    /// You can prevent the closing by listening to the close event.
    /// if force is equals true, then the close event will be ignored.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincloseforce)
    ///
    pub fn close_impl(this:&Window, force:bool);

    #[wasm_bindgen(method)]
    /// Reloads the current window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winreload)
    ///
    pub fn reload(this:&Window);

    #[wasm_bindgen(method, js_name=reloadDev)]
    /// Reloads the current page by starting a new renderer process from scratch.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winreloaddev)
    ///
    pub fn reload_dev(this:&Window);

    #[wasm_bindgen(method, js_name=reloadIgnoringCache)]
    /// Like reload(), but don’t use caches (aka “shift-reload”).
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winreloadignoringcache)
    ///
    pub fn reload_ignoring_cache(this:&Window);

    #[wasm_bindgen(method)]
    /// Maximize the window on GTK and Windows, and zoom the window on Mac OS X.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winmaximize)
    ///
    pub fn maximize(this:&Window);

    #[wasm_bindgen(method)]
    /// Unmaximize the window, i.e. the reverse of maximize().
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winunmaximize)
    ///
    pub fn unmaximize(this:&Window);

    #[wasm_bindgen(method)]
    /// Minimize the window to task bar on Windows, iconify the window on GTK,
    /// and miniaturize the window on Mac OS X.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winminimize)
    ///
    pub fn minimize(this:&Window);
    
    #[wasm_bindgen(method)]
    /// Restore window to previous state after the window is minimized,
    /// i.e. the reverse of minimize().
    /// It’s not named unminimize since restore is used commonly.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winrestore)
    ///
    pub fn restore(this:&Window);

    #[wasm_bindgen(method, js_name=enterFullscreen)]
    /// Make the window fullscreen.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winenterfullscreen)
    ///
    pub fn enter_fullscreen(this:&Window);

    #[wasm_bindgen(method, js_name=leaveFullscreen)]
    /// Leave the fullscreen mode.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winleavefullscreen)
    ///
    pub fn leave_fullscreen(this:&Window);

    #[wasm_bindgen(method, js_name=toggleFullscreen)]
    /// Toggle the fullscreen mode.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wintogglefullscreen)
    ///
    pub fn toggle_fullscreen(this:&Window);

    #[wasm_bindgen(method, js_name=enterKioskMode)]
    /// Enter the Kiosk mode.
    /// In Kiosk mode, the app will be fullscreen and try to prevent users from
    /// leaving the app, so you should remember to provide a way in app to
    /// leave Kiosk mode. This mode is mainly used for presentation on public
    /// displays.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winenterkioskmode)
    ///
    pub fn enter_kiosk_mode(this:&Window);

    #[wasm_bindgen(method, js_name=leaveKioskMode)]
    /// Leave the Kiosk mode.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winleavekioskmode)
    ///
    pub fn leave_kiosk_mode(this:&Window);

    #[wasm_bindgen(method, js_name=toggleKioskMode)]
    /// Toggle the kiosk mode.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wintogglekioskmode)
    ///
    pub fn toggle_kiosk_mode(this:&Window);

    #[wasm_bindgen(method, js_name=setTransparent)]
    /// Turn on/off the transparency support.
    /// 
    /// See more info on [Transparent Window](https://docs.nwjs.io/en/latest/For%20Users/Advanced/Transparent%20Window/).
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wintogglekioskmode)
    ///
    pub fn set_transparent(this:&Window, transparent:bool);

    #[wasm_bindgen(method, js_name=setShadow)]
    /// (Mac) Turn the window’s native shadow on/off.
    /// Useful for frameless, transparent windows.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetshadowshadow-mac)
    ///
    pub fn set_shadow(this:&Window, shadow:bool);

    #[wasm_bindgen(method, js_name=showDevTools)]
    /// Open the devtools to inspect the window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winshowdevtoolsiframe-callback)
    ///
    pub fn show_dev_tools(this:&Window);

    #[wasm_bindgen(method, js_name=showDevTools)]
    /// Open the devtools to inspect the window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winshowdevtoolsiframe-callback)
    ///
    pub fn show_dev_tools_with_id(this:&Window, iframe_id:&str);

    #[wasm_bindgen(method, js_name=showDevTools)]
    /// Open the devtools to inspect the window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winshowdevtoolsiframe-callback)
    ///
    pub fn show_dev_tools_with_id_and_callback(this:&Window, iframe_id:&str, callback:&Function);

    #[wasm_bindgen(method, js_name=showDevTools)]
    /// Open the devtools to inspect the window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winshowdevtoolsiframe-callback)
    ///
    pub fn show_dev_tools_with_iframe(this:&Window, iframe_element:&HtmlIFrameElement);

    #[wasm_bindgen(method, js_name=showDevTools)]
    /// Open the devtools to inspect the window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winshowdevtoolsiframe-callback)
    ///
    pub fn show_dev_tools_with_iframe_and_callback(this:&Window, iframe_element:&HtmlIFrameElement, callback:&Function);

    #[wasm_bindgen(method, js_name=closeDevTools)]
    /// Close the devtools window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winclosedevtools)
    ///
    pub fn close_dev_tools(this:&Window);

    #[wasm_bindgen(method, js_name=getPrinters)]
    /// Enumerate the printers in the system.
    /// The callback function will receive an array of JSON objects for
    /// the printer information. The device name of the JSON object can 
    /// be used as parameter in `win.print()`
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wingetprinterscallback)
    ///
    pub fn get_printers(this:&Window, callback:&Function);

    #[wasm_bindgen(method, js_name=isDevToolsOpen)]
    /// Query the status of devtools window.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winisdevtoolsopen)
    ///
    pub fn is_dev_tools_open(this:&Window)->bool;

    #[wasm_bindgen(method, js_name=print)]
    /// Print the web contents in the window with or without the need for 
    /// user’s interaction. 
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    ///
    pub fn print(this:&Window, options:&PrintOptions);

    #[wasm_bindgen(method, js_name=setMaximumSize)]
    /// Set window’s maximum size.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetmaximumsizewidth-height)
    ///
    pub fn set_maximum_size(this:&Window, width:u32, height:u32);

    #[wasm_bindgen(method, js_name=setMinimumSize)]
    /// Set window’s minimum size.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetminimumsizewidth-height)
    ///
    pub fn set_minimum_size(this:&Window, width:u32, height:u32);

    #[wasm_bindgen(method, js_name=setResizable)]
    /// Set whether window is resizable.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetresizableresizable)
    ///
    pub fn set_resizable(this:&Window, resizable:bool);

    #[wasm_bindgen(method, js_name=setAlwaysOnTop)]
    /// Sets the widget to be on top of all other windows in the window system.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetalwaysontoptop)
    ///
    pub fn set_always_on_top(this:&Window, top:bool);

    #[wasm_bindgen(method, js_name=setVisibleOnAllWorkspaces)]
    /// (Mac and Linux)
    /// Sets the widget to be on top of all other windows in the window system.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetvisibleonallworkspacesvisible-mac-and-linux)
    ///
    pub fn set_visible_on_all_workspaces(this:&Window, top:bool);

    #[wasm_bindgen(method, js_name=canSetVisibleOnAllWorkspaces)]
    /// (Mac and Linux)
    /// Returns a boolean indicating if the platform (currently Mac OS X and Linux) 
    /// support Window API method `win.set_visible_on_all_workspaces(true/false)`.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincansetvisibleonallworkspaces-mac-and-linux)
    ///
    pub fn can_set_visible_on_all_workspaces(this:&Window)->bool;

    #[wasm_bindgen(method, js_name=setPosition)]
    /// Move window to specified position.
    /// Currently only center is supported on all platforms,
    /// which will put window in the middle of the screen.
    /// 
    /// There are three valid positions: null or center or mouse
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetpositionposition)
    ///
    pub fn set_position_impl(this:&Window, position:JsValue);

    #[wasm_bindgen(method, js_name=setShowInTaskbar)]
    /// Control whether to show window in taskbar or dock.
    /// 
    /// See also `show_in_taskbar` in [Manifest-format](https://docs.nwjs.io/en/latest/References/Manifest%20Format/#show_in_taskbar).
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetshowintaskbarshow)
    ///
    pub fn set_show_in_taskbar(this:&Window, show:bool);

    #[wasm_bindgen(method, js_name=requestAttention)]
    /// Request the user’s attension by making the window flashes in the task bar.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winrequestattentionattension)
    ///
    pub fn request_attention(this:&Window, attension:bool);

    #[wasm_bindgen(method, js_name=requestAttention)]
    /// Request the user’s attension by making the window flashes in the task bar.
    /// 
    /// On Mac, value < 0 will trigger NSInformationalRequest, while value > 0 will trigger NSCriticalRequest.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winrequestattentionattension)
    ///
    pub fn request_attention_with_number(this:&Window, attension:i16);

    #[wasm_bindgen(method, js_name=capturePage)]
    /// Captures the visible area of the window.
    /// 
    /// To capture the full page, 
    /// see [win.captureScreenshot](https://docs.nwjs.io/en/latest/References/Window/#wincapturescreenshotoptions-callback).
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincapturepagecallback-config)
    ///
    pub fn capture_page(this:&Window, callback:&Function);

    #[wasm_bindgen(method, js_name=capturePage)]
    /// Captures the visible area of the window.
    /// 
    /// To capture the full page, 
    /// see [win.captureScreenshot](https://docs.nwjs.io/en/latest/References/Window/#wincapturescreenshotoptions-callback).
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincapturepagecallback-config)
    ///
    pub fn capture_page_with_config(
        this:&Window,
        callback:&Function,
        config:&CaptureConfig
    );

    #[wasm_bindgen(method, js_name=captureScreenshot)]
    /// Captures the the window.
    /// It can be used to capture the full page beyond the visible area.
    /// 
    /// Note: This API is experimental and subject to change in the future.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincapturepagecallback-config)
    ///
    pub fn capture_screenshot(this:&Window, config:&ScreenshotConfig)->Promise;

    #[wasm_bindgen(method, js_name=captureScreenshot)]
    /// Captures the the window.
    /// It can be used to capture the full page beyond the visible area.
    /// 
    /// Note: This API is experimental and subject to change in the future.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincapturepagecallback-config)
    ///
    pub fn capture_screenshot_with_callback(
        this:&Window,
        config:&ScreenshotConfig,
        callback:&Function
    );

    #[wasm_bindgen(method, js_name=setProgressBar)]
    /// Set progress bar
    /// 
    /// Note: Only Ubuntu is supported,
    /// and you’ll need to specify the application `.desktop` file through 
    /// `NW_DESKTOP` env. 
    /// 
    /// If `NW_DESKTOP` env variable is not found, it uses `nw.desktop` by default.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetprogressbarprogress)
    ///
    pub fn set_progress_bar(this:&Window, progress:f32);

    #[wasm_bindgen(method, js_name=setBadgeLabel)]
    /// Set the badge label on the window icon in taskbar or dock.
    /// 
    /// Note: This API is only supported on Ubuntu and the label is restricted 
    /// to a string number only. You’ll also need to specify the `.desktop` 
    /// file for your application (see the note on `set_progress_bar`)
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetbadgelabellabel)
    ///
    pub fn set_badge_label(this:&Window, label:&str);

    #[wasm_bindgen(method, js_name=eval)]
    /// Execute a piece of JavaScript in the frame.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winevalframe-script)
    ///
    pub fn eval_impl(this:&Window, iframe:JsValue, script:&str);

    #[wasm_bindgen(method, js_name=eval)]
    /// Execute a piece of JavaScript in the frame.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winevalframe-script)
    ///
    pub fn eval_with_iframe(this:&Window, iframe:&HtmlIFrameElement, script:&str);
    
    #[wasm_bindgen(method, js_name=evalNWBin)]
    /// Load and execute the compiled binary in the frame.
    /// 
    /// See [Protect JavaScript Source Code](https://docs.nwjs.io/en/latest/For%20Users/Advanced/Protect%20JavaScript%20Source%20Code/).
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winevalnwbinframe-path)
    ///
    pub fn eval_nw_bin_impl(this:&Window, iframe:JsValue, script:JsValue);

    #[wasm_bindgen(method, js_name=evalNWBin)]
    /// Load and execute the compiled binary in the frame.
    /// 
    /// See [Protect JavaScript Source Code](https://docs.nwjs.io/en/latest/For%20Users/Advanced/Protect%20JavaScript%20Source%20Code/).
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winevalnwbinframe-path)
    ///
    pub fn eval_nw_bin_with_iframe_impl(this:&Window, iframe:&HtmlIFrameElement, script:JsValue);

    #[wasm_bindgen(method, js_name=evalNWBinModule)]
    /// Load and execute the compiled binary for Modules in the frame.
    /// 
    /// The binary should be compiled with nwjc --nw-module.
    /// The following code will load `lib.bin` as module and other modules
    /// can refer to it with something like `import * from "./lib.js"`
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winevalnwbinmoduleframe-path-module_path)
    ///
    pub fn eval_nw_bin_module_impl(this:&Window, iframe:JsValue, script:JsValue, module_path:&str);

    #[wasm_bindgen(method, js_name=evalNWBinModule)]
    /// Load and execute the compiled binary for Modules in the frame.
    /// 
    /// The binary should be compiled with nwjc --nw-module.
    /// The following code will load `lib.bin` as module and other modules
    /// can refer to it with something like `import * from "./lib.js"`
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winevalnwbinmoduleframe-path-module_path)
    ///
    pub fn eval_nw_bin_module_with_iframe(
        this:&Window,
        iframe:&HtmlIFrameElement,
        script:JsValue,
        module_path:&str
    );

    #[wasm_bindgen(method, js_name=removeAllListeners)]
    /// Removes all listeners
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winremovealllistenerseventname)
    ///
    pub fn remove_all_listeners(this:&Window);

    #[wasm_bindgen(method, js_name=removeAllListeners)]
    /// Removes all listeners of the specified `event_name`
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winremovealllistenerseventname)
    ///
    pub fn remove_all_listeners_with_name(this:&Window, event_name:&str);


    #[wasm_bindgen(method)]
    /// Add event listener to the specified `event_name`
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#event-close)
    ///
    pub fn on(this:&Window, event_name:&str, callback:&Function);

}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace=["nw", "Window"], js_name = get)]
    /// Get current window.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowgetwindow_object)
    ///
    pub fn get() -> Window;

    #[wasm_bindgen(js_namespace=["nw", "Window"], js_name = getAll)]
    /// Get all windows with a callback function whose parameter is an array of nw::Window object.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowgetallcallback)
    ///
    pub fn get_all(callback:&Function);

    #[wasm_bindgen(js_namespace=["nw", "Window"], js_name = open)]
    /// Open new window
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    ///
    pub fn open(url:&str);

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Options;

    #[wasm_bindgen(js_namespace=["nw", "Window"], js_name = open)]
    /// Open window with options
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    pub fn open_with_options(url:&str, option:&Options);

    #[wasm_bindgen(js_namespace=["nw", "Window"], js_name = open)]
    /// Open window with options and callback.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#windowopenurl-options-callback)
    pub fn open_with_options_and_callback(url:&str, option:&Options, callback:&Function);


    /// Window Print options
    ///
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type PrintOptions;

    /// Window Capture Config
    ///
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type CaptureConfig;

    /// Screenshot Config
    ///
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type ScreenshotConfig;
    
}

/// Window position
/// 
pub enum WindowPosition{
    Null,
    Center,
    Mouse
}

/// NW Binary data
/// 
pub enum NWBinary{
    Path(String),
    ArrayBuffer(ArrayBuffer),
    //Buffer(Buffer)
}


impl Window{
    /// Set window’s menubar = null.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winmenu)
    ///
    pub fn remove_menu(&self){
        self.remove_menu_impl(JsValue::null());
    }

    /// Closes the current window without triggering `close` event.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincloseforce)
    ///
    pub fn close_with_force(&self){
        self.close_impl(true);
    }

    /// Move window to specified position.
    /// Currently only center is supported on all platforms,
    /// which will put window in the middle of the screen.
    /// 
    /// There are three valid positions: null or center or mouse
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winsetpositionposition)
    ///
    pub fn set_position(&self, position:WindowPosition){

        let position = match position{
            WindowPosition::Null => JsValue::null(),
            WindowPosition::Center => JsValue::from("center"),
            WindowPosition::Mouse => JsValue::from("mouse")
        };

        self.set_position_impl(position);
    }

    /// Execute a piece of JavaScript in the frame.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winevalframe-script)
    ///
    pub fn eval(&self, iframe:Option<HtmlIFrameElement>, script:&str){
        if let Some(iframe) = iframe{
            self.eval_with_iframe(&iframe, script);
        }else{
            self.eval_impl(JsValue::null(), script);
        }
    }

    /// Load and execute the compiled binary in the frame.
    /// 
    /// See [Protect JavaScript Source Code](https://docs.nwjs.io/en/latest/For%20Users/Advanced/Protect%20JavaScript%20Source%20Code/).
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winevalnwbinframe-path)
    ///
    pub fn eval_nw_bin(&self, iframe:Option<HtmlIFrameElement>, script:NWBinary){
        let script = match script{
            NWBinary::Path(path) => JsValue::from(path),
            NWBinary::ArrayBuffer(buffer) => JsValue::from(buffer)
        };
        if let Some(iframe) = iframe{
            self.eval_nw_bin_with_iframe_impl(&iframe, script);
        }else{
            self.eval_nw_bin_impl(JsValue::null(), script);
        }
    }

    /// Load and execute the compiled binary for Modules in the frame.
    /// 
    /// The binary should be compiled with nwjc --nw-module.
    /// The following code will load `lib.bin` as module and other modules
    /// can refer to it with something like `import * from "./lib.js"`
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winevalnwbinmoduleframe-path-module_path)
    ///
    pub fn eval_nw_bin_module(&self, iframe:Option<HtmlIFrameElement>, script:NWBinary, module_path:&str){
        let script = match script{
            NWBinary::Path(path) => JsValue::from(path),
            NWBinary::ArrayBuffer(buffer) => JsValue::from(buffer)
        };

        if let Some(iframe) = iframe{
            self.eval_nw_bin_module_with_iframe(&iframe, script, module_path);
        }else{
            self.eval_nw_bin_module_impl(JsValue::null(), script, module_path);
        }
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

impl OptionsExt for CaptureConfig{
    fn initialize(self)->Self{
        self.datatype("datauri")
    }
}

impl CaptureConfig{
    /// The image format used to generate the image.
    /// It supports two formats: "png" and "jpeg".
    /// 
    /// If ignored, it’s "jpeg" by default.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincapturepagecallback-config)
    pub fn format(self, format: &str) ->Self {
        self.set("format", JsValue::from(format))
    }

    /// It supports three types: "raw", "buffer" and "datauri".
    /// 
    /// If ignored, it’s "datauri" by default.
    /// 
    /// The `raw` only contains the Base64 encoded image.
    /// But `datauri` contains the mime type headers as well,
    /// and it can be directly assigned to src of Image to load the image.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincapturepagecallback-config)
    /// 
    pub fn datatype(self, datatype: &str) ->Self {
        self.set("datatype", JsValue::from(datatype))
    }
}


impl std::fmt::Display for CaptureConfig{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

impl OptionsExt for ScreenshotConfig{}

impl ScreenshotConfig{
    /// Capture the whole page beyond the visible area.
    /// Currently the height of captured image is capped at 16384 pixels by Chromium.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincapturescreenshotoptions-callback)
    /// 
    pub fn fullsize(self, fullsize: bool) ->Self {
        self.set("fullsize", JsValue::from(fullsize))
    }

    /// The image format used to generate the image.
    /// 
    /// It supports two formats: "png" and "jpeg".
    /// 
    /// "png" is the default.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincapturescreenshotoptions-callback)
    /// 
    pub fn format(self, format: &str) ->Self {
        self.set("format", JsValue::from(format))
    }

    /// Compression quality from range [0..100] (jpeg only).
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincapturescreenshotoptions-callback)
    /// 
    pub fn quality(self, quality: u8) ->Self {
        self.set("quality", JsValue::from(quality))
    }

    /// Capture the screenshot of a given region only.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#wincapturescreenshotoptions-callback)
    /// 
    pub fn clip(self, x:i32, y:i32, width:u32, height:u32, scale:u32) ->Self {
        let clip_region = Object::new();
        let items = vec![
            ("x", JsValue::from(x)),
            ("y", JsValue::from(y)),
            ("width", JsValue::from(width)),
            ("height", JsValue::from(height)),
            ("scale", JsValue::from(scale))
        ];
    
        for (key, value) in items{
            let _ = js_sys::Reflect::set(
                &clip_region,
                &JsValue::from(key),
                &value,
            );
        }
    
        self.set("clip", JsValue::from(clip_region))
    }
}

impl std::fmt::Display for ScreenshotConfig{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

/// Window print margin
pub enum PrintMargin{
    Default,
    NoMargins,
    Minimum,

    ///Custom margin: left, top, right, bottom
    Custom(Option<u16>, Option<u16>, Option<u16>, Option<u16>)
}

impl OptionsExt for PrintOptions{}

impl PrintOptions{
    /// Whether to print without the need for user’s interaction; optional,
    /// true by default
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn autoprint(self, autoprint: bool) ->Self {
        self.set("autoprint", JsValue::from(autoprint))
    }

    /// Hide the flashing print preview dialog; optional, false by default
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn silent(self, silent: bool) ->Self {
        self.set("silent", JsValue::from(silent))
    }

    /// The device name of the printer returned by `nw::Window::get_printers();`
    /// 
    /// No need to set this when printing to PDF
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn printer(self, printer: &str) ->Self {
        self.set("printer", JsValue::from(printer))
    }

    /// The path of the output PDF when printing to PDF
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn pdf_path(self, pdf_path: &str) ->Self {
        self.set("pdf_path", JsValue::from(pdf_path))
    }

    /// Whether to enable header and footer
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn header_footer_enabled(self, header_footer_enabled: bool) ->Self {
        self.set("headerFooterEnabled", JsValue::from(header_footer_enabled))
    }

    /// Whether to use landscape or portrait
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn landscape(self, landscape: bool) ->Self {
        self.set("landscape", JsValue::from(landscape))
    }

    /// The paper size spec
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn media_size(self, media_size: Object) ->Self {
        self.set("mediaSize", JsValue::from(media_size))
    }

    /// Whether to print CSS backgrounds
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn should_print_backgrounds(self, should_print_backgrounds: bool) ->Self {
        self.set("shouldPrintBackgrounds", JsValue::from(should_print_backgrounds))
    }

    /// MarginsType
    /// 
    /// see margins_custom.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn margin(mut self, margin: PrintMargin) ->Self {
        let margin_type = match margin{
            PrintMargin::Default=>0,
            PrintMargin::NoMargins=>1,
            PrintMargin::Minimum=>2,
            PrintMargin::Custom(l, t, r, b)=>{
                let margins_custom = Object::new();
                let items = vec![
                    ("marginLeft", l),
                    ("marginTop", t),
                    ("marginRight", r),
                    ("marginBottom", b),
                ];
            
                for (key, value) in items{
                    let v = value.unwrap_or(0);
                    let _ = js_sys::Reflect::set(
                        &margins_custom,
                        &JsValue::from(key),
                        &JsValue::from(v),
                    );
                }
                
                self = self.set("marginsCustom", JsValue::from(margins_custom));

                3
            }
        };
        self.set("marginsType", JsValue::from(margin_type))
    }

    /// The number of copies to print.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn copies(self, copies: u8) ->Self {
        self.set("copies", JsValue::from(copies))
    }

    /// The scale factor; 100 is the default.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn scale_factor(self, scale_factor: u8) ->Self {
        self.set("scaleFactor", JsValue::from(scale_factor))
    }

    /// String to replace the URL in the header.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn header_string(self, header_string: &str) ->Self {
        self.set("headerString", JsValue::from(header_string))
    }

    /// String to replace the URL in the footer.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Window/#winprintoptions)
    /// 
    pub fn footer_string(self, footer_string: &str) ->Self {
        self.set("footerString", JsValue::from(footer_string))
    }
}

impl std::fmt::Display for PrintOptions{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}


