use wasm_bindgen::prelude::*;
use js_sys::{Array, Function};
use workflow_log::log_info;
//use crate::options::OptionsExt;
use crate::result::Result;
use workflow_wasm::utils;


#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace=nw, js_name = Screen)]
    #[derive(Debug, Clone)]
    pub type Screen;
    
    #[wasm_bindgen(static_method_of=Screen, js_namespace=["nw"], js_name = Init)]
    /// Init the Screen singleton object, you only need to call this once
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screeninit)
    ///
    pub fn init();

    #[wasm_bindgen(getter, static_method_of=Screen, js_namespace=["nw"], js_name = screens)]
    fn screens_impl()->Array;

    #[wasm_bindgen(static_method_of=Screen, js_namespace=["nw"], js_name = chooseDesktopMedia)]
    fn choose_desktop_media_impl(sources:Array, callback:&Function);

    // Event: displayBoundsChanged(screen)
    // Event: displayAdded (screen)
    // Event: displayRemoved (screen)
    // Screen.DesktopCaptureMonitor
    // Synopsis
    // Screen.DesktopCaptureMonitor.started
    // Screen.DesktopCaptureMonitor.start(should_include_screens, should_include_windows)
    // Screen.DesktopCaptureMonitor.stop()
    // Screen.DesktopCaptureMonitor.registerStream(id)
    // Event: added (id, name, order, type, primary)
    // Event: removed (order)
    // Event: orderchanged (id, new_order, old_order)
    // Event: namechanged (id, name)
    // Event: thumbnailchanged (id, thumbnail)
    // Screen is an instance of EventEmitter object, and you’re able to use Screen.on(...) to respond to native screen’s events.
    
    // Screen is a singleton object, need to be initiated once by calling nw.Screen.Init()
    
    // Synopsis
    // //init must be called once during startup, before any function to nw.Screen can be called
    // nw.Screen.Init();
    
    // var screenCB = {
    //   onDisplayBoundsChanged: function(screen) {
    //     console.log('displayBoundsChanged', screen);
    //   },
    
    //   onDisplayAdded: function(screen) {
    //     console.log('displayAdded', screen);
    //   },
    
    //   onDisplayRemoved: function(screen) {
    //     console.log('displayRemoved', screen)
    //   }
    // };
    
    // // listen to screen events
    // nw.Screen.on('displayBoundsChanged', screenCB.onDisplayBoundsChanged);
    // nw.Screen.on('displayAdded', screenCB.onDisplayAdded);
    // nw.Screen.on('displayRemoved', screenCB.onDisplayRemoved);
    // Screen.Init()
    // Init the Screen singleton object, you only need to call this once
    
    // Screen.screens
    // Get the array of screen (number of screen connected to the computer)
    
    // screen has following structure:
    // screen {
    // // unique id for a screen
    //   id: int,
    
    // // physical screen resolution, can be negative, not necessarily start from 0,depending on screen arrangement
    //   bounds: {
    //     x: int,
    //     y: int,
    //     width: int,
    //     height: int
    //   },
    
    // // useable area within the screen bound
    //   work_area: {
    //     x: int,
    //     y: int,
    //     width: int,
    //     height: int
    //   },
    
    //   scaleFactor: float,
    //   isBuiltIn: bool,
    //   rotation: int,
    //   touchSupport: int
    // }
    // Screen.chooseDesktopMedia (sources, callback)
    // sources {String[]} array of source types. Two types are supported by this API: "screen" and "window".
    // callback {Function} callback function with chosed streamId. streamId will be false if failed to execute or existing session is alive.
    // Note
    
    // Screen sharing by selection; Currently only working in Windows and OSX and some linux distribution.
    
    // Example:
    
    // nw.Screen.Init(); // you only need to call this once
    // nw.Screen.chooseDesktopMedia(["window","screen"], 
    //   function(streamId) {
    //     var vid_constraint = {
    //       mandatory: {
    //         chromeMediaSource: 'desktop', 
    //         chromeMediaSourceId: streamId, 
    //         maxWidth: 1920, 
    //         maxHeight: 1080
    //       }, 
    //       optional: []
    //     };
    //     navigator.webkitGetUserMedia({audio: false, video: vid_constraint}, success_func, fallback_func);
    //   }
    // );
    // Event: displayBoundsChanged(screen)
    // Emitted when the screen resolution, arrangement is changed, the callback is called with 1 argument screen. See Screen.screens for the format.
    
    // Event: displayAdded (screen)
    // Emitted when a new screen added, the callback is called with 1 argument screen. See Screen.screens for the format.
    
    // Event: displayRemoved (screen)
    // Emitted when existing screen removed, the callback is called with 1 argument screen. See Screen.screens for the format.
    
    // Screen.DesktopCaptureMonitor
    // This API behaves similar functions as Screen.chooseDesktopMedia. But it doesn’t have GUI. You can use this API to monitor the changes of screens and windows on desktop and implement your own UI.
    
    // Screen.DesktopCaptureMonitor is an instance of EventEmitter. You can use Screen.DesktopCaptureMonitor.on() to listen to the events.
    
    // Synopsis
    // var dcm = nw.Screen.DesktopCaptureMonitor;
    // nw.Screen.Init();
    // dcm.on("added", function (id, name, order, type) {
    //    //select first stream and shutdown
    //    var constraints = {
    //       audio: {
    //          mandatory: {
    //              chromeMediaSource: "system",
    //              chromeMediaSourceId: dcm.registerStream(id)
    //           }
    //       },
    //       video: {
    //          mandatory: {
    //              chromeMediaSource: 'desktop',
    //              chromeMediaSourceId: dcm.registerStream(id)
    //          }
    //       }
    //   };
    
    //   // TODO: call getUserMedia with contraints
    
    //   dcm.stop();
    // });
    
    // dcm.on("removed", function (id) { });
    // dcm.on("orderchanged", function (id, new_order, old_order) { });
    // dcm.on("namechanged", function (id, name) { });
    // dcm.on("thumbnailchanged", function (id, thumbnail) { });
    // dcm.start(true, true);
    // Screen.DesktopCaptureMonitor.started
    // Boolean of whether the DesktopCaptureMonitor is started.
    
    // Screen.DesktopCaptureMonitor.start(should_include_screens, should_include_windows)
    // should_include_screens {Boolean} whether should include screens
    // should_include_windows {Boolean} whether should include windows
    // The DesktopCaptureMonitor will start monitoring the system and trigger the the events. The screen may flicker if while DesktopCaptureMonitor is running.
    
    // Screen.DesktopCaptureMonitor.stop()
    // The DesktopCaptureMonitor will stop monitoring the system. DesktopCaptureMonitor should be stopped after a stream is selected.
    
    // Screen.DesktopCaptureMonitor.registerStream(id)
    // Register and return a valid stream id passed into chromeMediaSourceId in getUserMedia constraints. See Synopsis for the usage.
    
    // Event: added (id, name, order, type, primary)
    // Behavior Changed
    
    // This feature is changed in 0.13.0. See Migration Notes from 0.12 to 0.13.
    
    // id {String} is the media id. Use registerStream(id) to obtain a valid stream id used with getUserMedia(). See registerStream
    // name {String} is the title of the window or screen
    // order {Integer} is the z-order of the windows, if screens are selected they will appear first
    // type {String} type of the stream: “screen”, “window”, “other” or “unknown”
    // primary {Boolean} Windows OS only this will be true if the source is the primary monitor
    // Emit when a new source was added.
    
    // Event: removed (order)
    // order {Integer} is the order of the media source that is no longer capturable
    // Emit when a source was removed.
    
    // Event: orderchanged (id, new_order, old_order)
    // Behavior Changed
    
    // This feature is changed in 0.13.0. See Migration Notes from 0.12 to 0.13.
    
    // id {String} is the media id of the screen or window that has changed z-order
    // new_order {Integer} is the new z-order
    // old_order {Integer} is the old z-order
    // Emit when the Z-order of a source changed (this may change for windows as others are focused).
    
    // Event: namechanged (id, name)
    // Behavior Changed
    
    // This feature is changed in 0.13.0. See Migration Notes from 0.12 to 0.13.
    
    // id {String} is the media id of the screen or window that has a name changed
    // name {String} is the new name of the screen or window
    // Emit when the name of the source has changed. This can happen when a window changes title.
    
    // Event: thumbnailchanged (id, thumbnail)
    // Behavior Changed
    
    // This feature is changed in 0.13.0. See Migration Notes from 0.12 to 0.13.
    
    // id {String} is the media id of the screen or window that has an updated thumbnail
    // thumbnail {String} is the base64 encoded png of the thumbnail
    // Emit when the thumbnail of a source changed.

}

pub enum Sources{
    Screen,
    Window,
    ScreenAndWindow
}
static mut INIT:bool = false;

impl Screen{

    pub fn is_initialized()->bool{
        unsafe{INIT}
    }

    pub fn init_once(){
        if !Self::is_initialized() {
            unsafe{INIT = true};
            Self::init();
        }
    }

    /// Get the array of screen (number of screen connected to the computer)
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screenscreens)
    ///
    pub fn screens()->Result<Vec<ScreenInfo>>{
        let mut result:Vec<ScreenInfo> = Vec::new();
        let array = Self::screens_impl();
        for index in 0..array.length(){
            let screen = array.get(index);
            //log_info!("screen: {:#?}", screen);
            result.push(screen.try_into()?);
        }
        Ok(result)
    }

    /// Get the array of screen (number of screen connected to the computer)
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screenchoosedesktopmedia-sources-callback)
    ///
    pub fn choose_desktop_media(sources:Sources, callback:&Function)->Result<()>{

        let array = Array::new();
        match sources {
            Sources::Screen=>{
                array.push(&JsValue::from("screen"));
            }
            Sources::Window=>{
                array.push(&JsValue::from("window"));
            }
            Sources::ScreenAndWindow=>{
                array.push(&JsValue::from("screen"));
                array.push(&JsValue::from("window"));
            }
        };

        Self::choose_desktop_media_impl(array, callback);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Bounds{
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
}

#[derive(Debug)]
pub struct WorkArea{
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64,
}

#[derive(Debug)]
pub struct ScreenInfo{
    pub id:u64,
    pub scale_factor:f64,
    pub is_built_in: bool,
    pub rotation: u64,
    pub touch_support: u64,
    pub bounds: Bounds,
    pub work_area: WorkArea
}

fn read_box(jsv: &JsValue, prop:&str)->Result<(u64, u64, u64, u64)>{
    let jsv = utils::try_get_js_value(jsv, prop)?;
    let x = utils::try_get_u64_from_prop(&jsv, "x")?;
    let y = utils::try_get_u64_from_prop(&jsv, "y")?;
    let width = utils::try_get_u64_from_prop(&jsv, "width")?;
    let height = utils::try_get_u64_from_prop(&jsv, "height")?;

    Ok((x, y, width, height))
}

impl TryFrom<JsValue> for ScreenInfo{
    type Error = crate::error::Error;
    fn try_from(jsv: JsValue) -> std::result::Result<Self, Self::Error> {

        let id = utils::try_get_u64_from_prop(&jsv, "id")?;
        let scale_factor = utils::try_get_f64_from_prop(&jsv, "scaleFactor")?;
        let is_built_in = utils::try_get_bool_from_prop(&jsv, "isBuiltIn")?;
        let rotation = utils::try_get_u64_from_prop(&jsv, "rotation")?;
        let touch_support = utils::try_get_u64_from_prop(&jsv, "touchSupport")?;

        let (x, y, width, height) = read_box(&jsv, "bounds")?;
        let bounds = Bounds{
            x, y, width, height
        };

        let (x, y, width, height) = read_box(&jsv, "work_area")?;
        let work_area = WorkArea{
            x, y, width, height
        };

        let info = Self {
            id,
            scale_factor,
            is_built_in,
            rotation,
            touch_support,
            bounds,
            work_area
        };
        Ok(info)
    }
}
