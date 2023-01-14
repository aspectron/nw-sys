//!
//! Functionality for building application, popup and tray menus.
//!
//! # Synopsis
//! ```rust
//! // An example to create a context menu:
//! // Create an empty context menu
//! let menu = nw_sys::Menu::new();
//!
//! // Add some items
//! menu.append(&nw_sys::MenuItem::new(&nw_sys::menu_item::Options::new().label("Item A")));
//! menu.append(&nw_sys::MenuItem::new(&nw_sys::menu_item::Options::new().label("Item B")));
//! menu.append(&nw_sys::MenuItem::new(&nw_sys::menu_item::Type::Separator.into()));
//! menu.append(&nw_sys::MenuItem::new(&nw_sys::menu_item::Options::new().label("Item C")));
//!
//! // Remove one item
//! menu.remove_at(1);
//!
//! // Popup as context menu
//! menu.popup(10, 10);
//!
//! // Iterate menu's items
//! for item in &menu.items(){
//!     log_trace!("{:?}", item);
//! }
//!
//! // To create a menubar, usually you have to create a 2-level menu and assign it
//! // to win.menu. Here is the example of creating a menubar:
//! //
//! // Create an empty menubar
//! let menu = nw_sys::Menu::new_with_options(&nw_sys::menu::Type::Menubar.into());
//!
//! // Create a submenu as the 2nd level menu
//! let submenu = nw_sys::Menu::new();
//! submenu.append(&nw_sys::MenuItem::new(&nw_sys::menu_item::Options::new().label("Item A")));
//! submenu.append(&nw_sys::MenuItem::new(&nw_sys::menu_item::Options::new().label("Item B")));
//!
//! // Create and append the 1st level menu to the menubar
//! let options = nw_sys::menu_item::Options::new()
//!     .label("First Menu")
//!     .submenu(&submenu);
//!
//! //create builtin Edit and Window menus on Mac
//! menu.create_mac_builtin("Example App");
//!
//! menu.append(&nw_sys::MenuItem::new(&options));
//!
//! // Assign it to `window.menu` to get the menu displayed
//! nw_sys::window::get().set_menu(&menu);
//!
//! ```

use crate::menu_item::MenuItem;
use js_sys::Object;
use wasm_bindgen::prelude::*;
use workflow_wasm::options::OptionsExt;

#[wasm_bindgen]
extern "C" {

    ///
    /// Menu interface. For usage example please refer to [nw_sys::menu](self)
    ///

    #[wasm_bindgen(js_namespace=nw, js_name = Menu)]
    #[derive(Debug, Clone)]
    pub type Menu;

    #[wasm_bindgen(constructor, js_namespace=["nw"])]
    /// Create Menu
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/)
    ///
    pub fn new() -> Menu;

    #[wasm_bindgen(constructor, js_namespace=["nw"])]
    /// Create Menu
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#new-menuoption)
    ///
    pub fn new_with_options(options: &Options) -> Menu;

    #[wasm_bindgen(method, getter, js_name = items)]
    /// Get an array that contains all items of a menu.
    /// Each item is an instance of MenuItem.
    /// See MenuItem for detailed information.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menuitems)
    pub fn items(this: &Menu) -> Vec<MenuItem>;

    #[wasm_bindgen(method)]
    /// Append item to the tail of the menu.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menuappenditem)
    ///
    pub fn append(this: &Menu, item: &MenuItem);

    #[wasm_bindgen(method, js_name = insert)]
    /// Insert the item at `index`th position of the menu. The index is started from 0.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menuinsertitem-i)
    pub fn insert(this: &Menu, item: &MenuItem, index: u16);

    #[wasm_bindgen(method, js_name = remove)]
    /// Remove item from the menu. This method requires you to keep the MenuItem outside the Menu.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menuremoveitem)
    pub fn remove(this: &Menu);

    #[wasm_bindgen(method, js_name = removeAt)]
    /// Remove the item form the menu by index.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menuremoveati)
    pub fn remove_at(this: &Menu, index: u16);

    #[wasm_bindgen(method, js_name = popup)]
    /// Popup the context menu at the anchor in (x, y) in current window.
    /// This method is only valid for contextmenu type.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menupopupx-y)
    pub fn popup(this: &Menu, x: i32, y: i32);

    #[wasm_bindgen(method, js_name=createMacBuiltin)]
    /// (Mac) Creates the builtin menus (App, Edit and Window) within the menubar on Mac.
    /// The items can be manipulated with the items property.
    /// The argument appname is used for the title of App menu.
    /// You can still use builtin menus with other menu items.
    /// i.e. append or insert items to the menu is still valid.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menucreatemacbuiltinappname-options-mac)
    ///
    pub fn create_mac_builtin(this: &Menu, app_name: &str);

    #[wasm_bindgen(method, js_name=createMacBuiltin)]
    /// (Mac) Creates the builtin menus (App, Edit and Window) within the menubar on Mac.
    /// The items can be manipulated with the items property.
    /// The argument appname is used for the title of App menu.
    /// You can still use builtin menus with other menu items.
    /// i.e. append or insert items to the menu is still valid.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menucreatemacbuiltinappname-options-mac)
    ///
    pub fn create_mac_builtin_with_options(this: &Menu, app_name: &str, options: &MacOptions);

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Options;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type MacOptions;

}

impl OptionsExt for Options {}
impl OptionsExt for MacOptions {}

impl MacOptions {
    /// do not populate the Edit menu
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menucreatemacbuiltinappname-options-mac)
    pub fn hide_edit(self, hide: bool) -> Self {
        self.set("hideEdit", JsValue::from(hide))
    }
    /// do not populate the Window menu
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menucreatemacbuiltinappname-options-mac)
    pub fn hide_window(self, hide: bool) -> Self {
        self.set("hideWindow", JsValue::from(hide))
    }
}

/// Menu Type
pub enum Type {
    Menubar,
}

impl From<Type> for Options {
    fn from(t: Type) -> Self {
        let options = Self::new();
        let options = match t {
            Type::Menubar => options.set("type", JsValue::from("menubar")),
        };

        options
    }
}
