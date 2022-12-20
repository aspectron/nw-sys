use wasm_bindgen::prelude::*;
use js_sys::{Array, Function};
//use workflow_log::log_info;
//use crate::options::OptionsExt;
use crate::result::Result;
use workflow_wasm::utils;


#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace=nw, js_name = Screen)]
    #[derive(Debug, Clone)]
    type ScreenLocal;
    
    
    #[wasm_bindgen(js_namespace=["nw", "Screen"], js_name = Init)]
    /// Init the Screen singleton object, you only need to call this once
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screeninit)
    ///
    pub fn init();

    #[wasm_bindgen(getter, static_method_of=ScreenLocal, js_namespace=["nw"], js_class=Screen, js_name = screens)]
    fn screens_impl()->Array;

    #[wasm_bindgen(js_namespace=["nw", "Screen"], js_name = chooseDesktopMedia)]
    fn choose_desktop_media_impl(sources:Array, callback:&Function);

    #[wasm_bindgen(js_namespace=["nw", "Screen"], js_name = on)]
    ///
    /// 
    /// ### Events:
    /// - displayBoundsChanged (screen)
    /// - displayAdded (screen)
    /// - displayRemoved (screen)
    /// 
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#event-displayboundschangedscreen)
    /// 
    pub fn on(event_name:&str, callback:&Function);



    #[wasm_bindgen(js_namespace=["nw", "Screen"], js_name = DesktopCaptureMonitor)]
    #[derive(Debug, Clone)]
    pub type DesktopCaptureMonitor;
    
    #[wasm_bindgen(static_method_of=DesktopCaptureMonitor, js_namespace=["nw", "Screen"], js_name = start)]
    /// The DesktopCaptureMonitor will start monitoring the system 
    /// and trigger the the events. The screen may flicker 
    /// if while DesktopCaptureMonitor is running.
    /// 
    /// Example:
    /// ```rust
    /// nw::screen::DesktopCaptureMonitor::start(true, true);
    /// ```
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screendesktopcapturemonitorstartshould_include_screens-should_include_windows)
    ///
    pub fn start(should_include_screens:bool, should_include_windows:bool);

    #[wasm_bindgen(getter, static_method_of=DesktopCaptureMonitor, js_namespace=["nw", "Screen"], js_name = started)]
    /// Return Boolean of whether the DesktopCaptureMonitor is started.
    /// 
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screendesktopcapturemonitorstarted)
    ///
    pub fn started()->bool;

    #[wasm_bindgen(static_method_of=DesktopCaptureMonitor, js_namespace=["nw", "Screen"], js_name = stop)]
    /// The `DesktopCaptureMonitor` will stop monitoring the system.
    /// `DesktopCaptureMonitor` should be stopped after a stream is selected.
    /// 
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screendesktopcapturemonitorstop)
    ///
    pub fn stop();

    #[wasm_bindgen(static_method_of=DesktopCaptureMonitor, js_namespace=["nw", "Screen"], js_name = registerStream)]
    /// Register and return a valid stream id which will be used into 
    /// chromeMediaSourceId in get_user_media constraints.
    /// 
    /// See Synopsis for the usage.
    /// 
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screendesktopcapturemonitorregisterstreamid)
    ///
    pub fn register_stream(id:&str)->String;


    #[wasm_bindgen(static_method_of=DesktopCaptureMonitor, js_namespace=["nw", "Screen"], js_name = on)]
    /// Add event listener
    /// 
    /// ### Events:
    /// - added (id, name, order, type, primary)
    /// - removed (order)
    /// - orderchanged (id, new_order, old_order)
    /// - namechanged (id, name)
    /// - thumbnailchanged (id, thumbnail)
    /// 
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#event-added-id-name-order-type-primary)
    ///
    pub fn on(event_name:&str, callback:&Function);


}

static mut INIT:bool = false;

/// Return true is screen is initialized
pub fn is_initialized()->bool{
    unsafe{INIT}
}

/// Call the screen::init() if screen is not initialized yet
pub fn init_once(){
    if !is_initialized() {
        unsafe{INIT = true};
        init();
    }
}

///Media source type
pub enum MediaSources{
    Screen,
    Window,
    ScreenAndWindow
}

/// Get the array of screen (number of screen connected to the computer)
/// 
/// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screenchoosedesktopmedia-sources-callback)
///
pub fn choose_desktop_media(sources:MediaSources, callback:&Function)->Result<()>{

    let array = Array::new();
    match sources {
        MediaSources::Screen=>{
            array.push(&JsValue::from("screen"));
        }
        MediaSources::Window=>{
            array.push(&JsValue::from("window"));
        }
        MediaSources::ScreenAndWindow=>{
            array.push(&JsValue::from("screen"));
            array.push(&JsValue::from("window"));
        }
    };

    choose_desktop_media_impl(array, callback);
    Ok(())
}

/// Get the array of screen (number of screen connected to the computer)
/// 
/// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Screen/#screenscreens)
///
pub fn screens()->Result<Vec<ScreenInfo>>{
    let mut result:Vec<ScreenInfo> = Vec::new();
    let array = ScreenLocal::screens_impl();
    for index in 0..array.length(){
        let screen = array.get(index);
        //log_info!("screen: {:#?}", screen);
        result.push(screen.try_into()?);
    }
    Ok(result)
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

/// Screen Info
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
