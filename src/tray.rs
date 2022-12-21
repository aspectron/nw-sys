//! nw.Tray
//! 
//! # Synopsis
//! ```
//! let options = nw_sys::tray::Options::new()
//!     //.title("My App")
//!     .icon("resources/icons/tray-icon@2x.png")
//!     .icons_are_templates(false);
//! 
//! let tray = nw_sys::Tray::new(&options);
//! tray.title();
//! tray.tooltip();
//! tray.icon();
//! tray.alticon();// (Mac)
//! tray.icons_are_templates();// (Mac)
//! tray.menu();
//! //tray.remove();
//! 
//! let menu = nw_sys::Menu::new();
//! menu.append(&nw_sys::MenuItem::new(&nw_sys::menu_item::Options::new().label("Menu 1")));
//! tray.set_menu(&menu);
//! 
//! ```
//! 
use wasm_bindgen::prelude::*;
use js_sys::{Object, Function};
use crate::options::OptionsExt;
use crate::menu::Menu;


#[wasm_bindgen]
extern "C" {
    ///
    /// For usage example please refer to [nw_sys::tray](self)
    /// 
    /// Tray is an abstraction of different controls on different platforms,
    /// usually it’s a small icon shown on the OS’s notification area.
    /// On Mac OS X it’s called Status Item, on GTK it’s Status Icon,
    /// and on Windows it’s System Tray Icon.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#tray)
    ///
    #[wasm_bindgen(js_namespace=nw, js_name = Tray)]
    #[derive(Debug, Clone)]
    pub type Tray;

    #[wasm_bindgen(constructor, js_namespace=["nw"])]
    /// # Synopsis
    /// ```
    /// //Create a tray icon
    /// let tray = nw_sys::Tray::new(&nw_sys::tray::Options::new().title("Tray").icon("img/icon.png"));
    /// ```
    ///
    /// Create a new Tray, option is an contains initial settings for the Tray.
    /// Every field has its own property in the Tray,
    /// see documentation of each property for details.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#tray)
    ///
    pub fn new(options:&Options) -> Tray; 

    #[wasm_bindgen(method, getter, js_name=title)]
    /// Get the title of the tray.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traytitle)
    pub fn title(this:&Tray)->String;

    #[wasm_bindgen(method, setter, js_name=title)]
    /// Set the title of the tray.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traytitle)
    pub fn set_title(this:&Tray, title:&str);


    #[wasm_bindgen(method, getter, js_name=tooltip)]
    /// Get the tooltip of the tray.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traytooltip)
    pub fn tooltip(this:&Tray)->String;

    #[wasm_bindgen(method, setter, js_name=tooltip)]
    /// Set the tooltip of the tray. tooltip shows when you hover the Tray with mouse.
    /// On Mac OS X title will be showed on status bar along with its icon, 
    /// but it doesn’t have effects on GTK and Windows, since the latter 
    /// two platforms only support tray to be showed as icons.
    ///
    /// Note:
    ///
    /// tooltip is showed on all three platforms. Should be set as Tray property 
    /// rather from option object constructor.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traytooltip)
    pub fn set_tooltip(this:&Tray, tooltip:&str);

    #[wasm_bindgen(method, getter, js_name=icon)]
    /// Get the icon of the tray.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayicon)
    pub fn icon(this:&Tray)->String;

    #[wasm_bindgen(method, setter, js_name=icon)]
    /// Set the icon of the tray. Icon must a path to your icon file.
    /// It can be a relative path which points to an icon in your app,
    /// or an absolute path pointing to a file in user’s system.
    ///
    /// 
    /// Mac OS X caveat: when used in notification context, 
    /// png icon is not sized down like in windows notification area, 
    /// it is rather displayed in 1:1 ratio.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayicon)
    pub fn set_icon(this:&Tray, icon:&str);
    

    #[wasm_bindgen(method, getter, js_name=alticon)]
    /// (Mac) Get the alternate (active) tray icon.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayalticon-mac)
    pub fn alticon(this:&Tray)->String;

    #[wasm_bindgen(method, setter, js_name=alticon)]
    /// (Mac) Set the alternate (active) tray icon.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayalticon-mac)
    pub fn set_alticon(this:&Tray, alticon:&str);

    #[wasm_bindgen(method, getter, js_name=iconsAreTemplates)]
    /// (Mac) Get whether icon and alticon images are treated as “templates” 
    /// (true by default). When the property is set to true the images are 
    /// treated as “templates” and the system automatically ensures proper 
    /// styling according to the various states of the status item 
    /// (e.g. dark menu, light menu, etc.). 
    /// Template images should consist only of black and clear colours and 
    /// can use the alpha channel in the image to adjust the opacity of 
    /// black content.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayiconsaretemplates-mac)
    pub fn icons_are_templates(this:&Tray)->bool;

    #[wasm_bindgen(method, setter, js_name=alticon)]
    /// (Mac) Set whether icon and alticon images are treated as “templates” 
    /// (true by default).
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayalticon-mac)
    pub fn set_icons_are_templates(this:&Tray, value:bool);

    #[wasm_bindgen(method, getter, js_name = menu)]
    /// Get the menu of the tray
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traymenu)
    pub fn menu(this:&Tray)->Option<Menu>;

    #[wasm_bindgen(method, setter, js_name = menu)]
    /// Set the menu of the tray, menu will be showed when you click on the tray icon.
    /// 
    /// ```rust
    /// let menu = nw_sys::Menu::new();
    /// menu.append(&nw_sys::MenuItem::new(&nw_sys::menu_item::Options::new().label("Menu 1")));
    /// tray.set_menu(&menu);
    /// ```
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traymenu)
    pub fn set_menu(this:&Tray, menu:&Menu);
    
    #[wasm_bindgen(method)]
    /// Remove the tray
    /// 
    ///
    /// tray = null;<---- TODO
    /// 
    /// Once removed, you will not be able to show it again and you should set 
    /// your tray variable to null to make it garbage collected.
    /// There is no way temporarily hide a tray icon.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayremove)
    pub fn remove(this:&Tray);
    
    #[wasm_bindgen(method, js_name=on)]
    /// Event handling: Click
    /// 
    /// Emitted when user clicks the menu item with left mouse button.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#event-click)
    pub fn on(this:&Tray, event:&str, callback:&Function);

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Options;
}


impl OptionsExt for Options{}

impl Options{
    /// Set the title of the tray.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traytitle)
    pub fn title(self, title:&str)->Self{
        self.set("title", JsValue::from(title))
    }

    /// Set the tooltip of the tray. tooltip shows when you hover the Tray with mouse.
    /// 
    /// Note: tooltip is showed on all three platforms.
    /// Should be set as Tray property rather from option object constructor.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traytooltip)
    pub fn tooltip(self, tooltip:&str)->Self{
        self.set("tooltip", JsValue::from(tooltip))
    }

    /// Set the icon of the tray, icon must a path to your icon file.
    /// It can be a relative path which points to an icon in your app,
    /// or an absolute path pointing to a file in user’s system.
    /// 
    /// Mac OS X caveat: when used in notification context,
    /// png icon is not sized down like in windows notification area,
    /// it is rather displayed in 1:1 ratio.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayicon)
    pub fn icon(self, icon:&str)->Self{
        self.set("icon", JsValue::from(icon))
    }

    /// (Mac) Set the alternate (active) tray icon.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayalticon-mac)
    pub fn alticon(self, alticon:&str)->Self{
        self.set("alticon", JsValue::from(alticon))
    }

    /// (Mac) Set whether icon and alticon images are treated as "templates" (true by default).
    /// When the property is set to true the images are treated as “templates”
    /// and the system automatically ensures proper styling according to the various
    /// states of the status item (e.g. dark menu, light menu, etc.).
    /// Template images should consist only of black and clear colours
    /// and can use the alpha channel in the image to adjust the opacity of black content.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#trayiconsaretemplates-mac)
    pub fn icons_are_templates(self, icons_are_templates:bool)->Self{
        self.set("iconsAreTemplates", JsValue::from(icons_are_templates))
    }

    /// Set the menu of the tray, menu will be showed when you click on the tray icon.
    /// 
    /// On Mac OS X the menu will be showed when you click on the 
    /// tray (which is the only action available for tray icons on Mac OS X).
    /// On Windows and Linux, the menu will be showed when you single click on the 
    /// tray with right mouse button, clicking with left mouse button sends the click 
    /// event and does not show a menu.
    /// 
    /// In order to reduce differences from different platforms, setting menu property 
    /// is the only way to bind a menu to tray, there’s no way to popup a menu with 
    /// left mouse button click on Linux and Windows.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Tray/#traymenu)
    pub fn menu(self, menu: &Menu)->Self{
        self.set("menu", JsValue::from(menu))
    }

}
