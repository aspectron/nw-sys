//!
//! App bindings providing application control, access to command-line arguments,
//! working directory, paths for the browser data folders, cache control, origin control,
//! global keyboard hotkey registration, proxy control, underlaying browser control and 
//! `package.json` manifest access.
//! 

use wasm_bindgen::prelude::*;
use js_sys::{Object, Array, RegExp, Function};
use crate::result::Result;
use crate::shortcut::Shortcut;


#[wasm_bindgen]
extern "C" {


    #[wasm_bindgen(js_namespace=nw, js_name = App)]
    type NwApp;

    /// (Internal) Get the filtered command line arguments when starting the app.
    /// In NW.js, some command line arguments are used by NW.js,
    /// which should not be interested of your app. App.argv will filter out 
    /// those arguments and return the ones left. You can get filtered patterns 
    /// from [app::filtered_argv](self::filtered_argv) and the full arguments from [app::full_argv](self::full_argv).
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appargv)
    /// 
    #[wasm_bindgen(getter, static_method_of=NwApp, js_namespace=nw, js_class=App, js_name = argv)]
    pub fn argv_impl() -> Array;

    /// (Internal) Get all the command line arguments when starting the app.
    /// The return values contains the arguments used by NW.js,
    /// such as --nwapp, --remote-debugging-port etc.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appfullargv)
    #[wasm_bindgen(getter, static_method_of=NwApp, js_namespace=nw, js_class=App, js_name = fullArgv)]
    pub fn full_argv_impl() -> Array;

    /// (Internal) Get a list of patterns of filtered command line arguments used by App.argv.
    /// By default, following patterns are used to filter the arguments:
    /// ```
    /// [
    ///     /^--url=/,
    ///     /^--remote-debugging-port=/,
    ///     /^--renderer-cmd-prefix=/,
    ///     /^--nwapp=/
    /// ]
    /// ```
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appfilteredargv)
    #[wasm_bindgen(getter, static_method_of=NwApp, js_namespace=nw, js_class=App, js_name = filteredArgv)]
    pub fn filtered_argv_impl() -> Array;

    #[wasm_bindgen(getter, static_method_of=NwApp, js_namespace=nw, js_class=App, js_name = startPath)]
    fn start_path() -> String;

    #[wasm_bindgen(getter, static_method_of=NwApp, js_namespace=nw, js_class=App, js_name = dataPath)]
    fn data_path() -> String;


    #[wasm_bindgen(getter, static_method_of=NwApp, js_namespace=nw, js_class=App, js_name = manifest)]
    fn manifest() -> Object;

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = clearCache)]
    /// Clear the HTTP cache in memory and the one on disk.
    /// This method call is synchronized.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appclearcache)
    ///
    pub fn clear_cache();

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = clearAppCache)]
    /// Mark the Application cache group specified by `manifest_url` obsolete.
    /// This method call is synchronized.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appclearappcachemanifest_url)
    ///
    pub fn clear_app_cache(manifest_url:&str);

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = closeAllWindows)]
    /// Send the close event to all windows of current app,
    /// if no window is blocking the close event, then the app will quit after
    /// all windows have done shutdown. Use this method to quit an app 
    /// will give windows a chance to save data.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appcloseallwindows)
    ///
    pub fn close_all_windows();

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = crashBrowser)]
    /// Crashes the browser process to test 
    /// the [Crash dump](https://docs.nwjs.io/en/latest/For%20Developers/Understanding%20Crash%20Dump/) feature.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appcrashbrowser)
    ///
    pub fn crash_browser();

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = crashRenderer)]
    /// Crashes the renderer process to test 
    /// the [Crash dump](https://docs.nwjs.io/en/latest/For%20Developers/Understanding%20Crash%20Dump/) feature.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appcrashrenderer)
    ///
    pub fn crash_renderer();

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = enableComponent)]
    /// Experimental
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appenablecomponentcomponent-callback)
    ///
    pub fn enable_component(component:&str, callback:&Function);

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = getProxyForURL)]
    /// Query the proxy to be used for loading `url` in DOM.
    /// The return value is in the same format used in 
    /// PAC (e.g. "DIRECT", "PROXY localhost:8080").
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appgetproxyforurlurl)
    ///
    fn get_proxy_for_url(url:&str)->String;

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = setProxyConfig)]
    /// Set the proxy config which the web engine will be used to request 
    /// network resources or PAC url to detect proxy automatically.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appsetproxyconfigconfig-pac_url)
    ///
    pub fn set_proxy_config(config:&str, pac_url:&str);

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = quit)]
    /// Quit current app. 
    /// This method will not send `close` event to windows and app will 
    /// just quit quietly.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appquit)
    ///
    pub fn quit();

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = setCrashDumpDir)]
    /// Deprecated: Set the directory where the minidump 
    /// file will be saved on crash. For more information, 
    /// see [Crash dump](https://docs.nwjs.io/en/latest/For%20Developers/Understanding%20Crash%20Dump/).
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appsetcrashdumpdirdir)
    ///
    pub fn set_crash_dump_dir(dir:&str);

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = addOriginAccessWhitelistEntry)]
    /// Add an entry to the whitelist used for controlling cross-origin access.
    /// Suppose you want to allow HTTP redirecting from github.com to 
    /// the page of your app, use something like this:
    ///
    /// ```
    /// nw_sys::app::add_origin_access_whitelist_entry(
    ///     "http://github.com/",
    ///     "chrome-extension",
    ///     "domain.com",
    ///     true
    /// );
    /// ```
    /// Use [nw_sys::app::remove_origin_access_whitelist_entry()](self::remove_origin_access_whitelist_entry) with exactly the 
    /// same arguments to do the contrary.
    /// 
    /// - `source_origin`: The source origin. e.g. http://github.com/
    /// - `destination_protocol`: The destination protocol where the `source_origin` can access to. e.g. `app`
    /// - `destination_host`: The destination host where the `source_origin` can access to. e.g. `myapp`
    /// - `allow_destination_subdomains`: If set to true, the `source_origin` is allowed to access subdomains of destinations.
    /// 
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appaddoriginaccesswhitelistentrysourceorigin-destinationprotocol-destinationhost-allowdestinationsubdomains)
    ///
    pub fn add_origin_access_whitelist_entry(
        source_origin: &str,
        destination_protocol: &str,
        destination_host: &str,
        allow_destination_subdomains: bool
    );

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = removeOriginAccessWhitelistEntry)]
    /// Remove an entry from the whitelist used for controlling cross-origin access.
    /// See [nw_sys::app::add_origin_access_whitelist_entry()](self::add_origin_access_whitelist_entry).
    /// 
    /// 
    /// - `source_origin`: The source origin. e.g. http://github.com/
    /// - `destination_protocol`: The destination protocol where the `source_origin` can access to. e.g. `app`
    /// - `destination_host`: The destination host where the `source_origin` can access to. e.g. `myapp`
    /// - `allow_destination_subdomains`: If set to true, the `source_origin` is allowed to access subdomains of destinations.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appremoveoriginaccesswhitelistentrysourceorigin-destinationprotocol-destinationhost-allowdestinationsubdomains)
    ///
    pub fn remove_origin_access_whitelist_entry(
        source_origin: &str,
        destination_protocol: &str,
        destination_host: &str,
        allow_destination_subdomains: bool
    );

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = registerGlobalHotKey)]
    /// Register a global keyboard shortcut (also known as system-wide hot key)
    /// to the system.
    /// 
    /// See [Shortcut](crate::shortcut) for more information.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appregisterglobalhotkeyshortcut)
    ///
    pub fn register_global_hot_key(shortcut:&Shortcut);

    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = unregisterGlobalHotKey)]
    /// Unregisters a global keyboard shortcut.
    /// 
    /// See [Shortcut](crate::shortcut) for more information.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appunregisterglobalhotkeyshortcut)
    ///
    pub fn unregister_global_hot_key(shortcut:&Shortcut);


    #[wasm_bindgen(js_namespace=["nw", "App"], js_name = updateComponent)]
    /// Experimental
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appupdatecomponentcomponent-callback)
    ///
    pub fn update_component(component:&str, callback:&Function);

    // Event: open(args:String)
    // args: the full command line of the program.
    // Emitted when users opened a file with your app.

    // Event: reopen
    // This is a Mac specific feature. 
    // This event is sent when the user clicks the dock icon for an 
    // already running application.

}

fn build_argv_str(argv:Array)->Result<Vec<String>>{
    let mut list = Vec::new();
    for index in 0..argv.length(){
        match argv.get(index).as_string(){
            Some(v)=>{
                list.push(v);
            }
            None=>{}
        }
        
    }

    Ok(list)
}

fn build_argv_filters(argv:Array)->Result<Vec<RegExp>>{
    let mut list = Vec::new();
    for index in 0..argv.length(){
        let a = argv.get(index);
        let v = RegExp::from(a);
        list.push(v);
        /*
        match argv.get(index).as_string(){
            Some(v)=>{
                list.push(v);
            }
            None=>{}
        }
        */
    }

    Ok(list)
}

/// Get the filtered command line arguments when starting the app.
/// In NW.js, some command line arguments are used by NW.js,
/// which should not be interested of your app. App.argv will filter out 
/// those arguments and return the ones left. You can get filtered patterns 
/// from [app::filtered_argv](self::filtered_argv) and the 
/// full arguments from [app::full_argv](self::full_argv).
/// 
/// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appargv)
///
pub fn argv()->Result<Vec<String>>{
    let list = build_argv_str(NwApp::argv_impl())?;
    Ok(list)
}

/// Get all the command line arguments when starting the app.
/// The return values contains the arguments used by NW.js,
/// such as --nwapp, --remote-debugging-port etc.
/// 
/// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appfullargv)
pub fn full_argv()->Result<Vec<String>>{
    let list = build_argv_str(NwApp::full_argv_impl())?;
    Ok(list)
}

/// Get a list of patterns of filtered command line arguments used by [app::argv()](self::argv).
/// By default, following patterns are used to filter the arguments:
/// ```
/// [
///     /^--url=/,
///     /^--remote-debugging-port=/,
///     /^--renderer-cmd-prefix=/,
///     /^--nwapp=/
/// ]
/// ```
/// 
/// 
/// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appfilteredargv)
pub fn filtered_argv()->Result<Vec<RegExp>>{
    let list = build_argv_filters(NwApp::filtered_argv_impl())?;
    Ok(list)
}


/// Get the directory where the application starts.
/// The application will change the current directory to where 
/// the package files reside after start.
/// 
/// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appstartpath)
///
/// 
pub fn start_path()->String{
    NwApp::start_path()
}

/// Get the application’s data path in user’s directory.
/// 
/// - Windows: `%LOCALAPPDATA%/<name>`
/// - Linux: `~/.config/<name>`
/// - OS X: `~/Library/Application Support/<name>/Default`
/// - `<name>` is the name field in the `package.json` manifest.
/// 
/// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appdatapath)
///
pub fn data_path() -> String{
    NwApp::data_path()
}

/// Get the JSON object of the manifest file.
/// 
/// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/App/#appmanifest)
///
pub fn manifest() -> Object{
    NwApp::manifest()
}
