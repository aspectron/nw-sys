use wasm_bindgen::prelude::*;
use js_sys::Object;
use crate::menu_item::MenuItem;
use crate::options::OptionsExt;


#[wasm_bindgen]
extern "C" {

    // Menu
    // Synopsis
    // nw::Menu::new()

    #[wasm_bindgen(js_namespace=nw, js_name = Menu)]
    #[derive(Debug, Clone)]
    pub type Menu;

    #[wasm_bindgen(constructor, js_namespace=["nw"])]
    /// Create Menu
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/)
    ///
    pub fn new() -> Menu;

    #[wasm_bindgen(constructor, js_namespace=["nw"])]
    /// Create Menu
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#new-menuoption)
    ///
    pub fn new_with_options(options:&Options) -> Menu;

    #[wasm_bindgen(method, getter, js_name = items)]
    /// Get an array that contains all items of a menu.
    /// Each item is an instance of MenuItem.
    /// See MenuItem for detailed information.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menuitems)
    pub fn items(this:&Menu)->Vec<MenuItem>;

    #[wasm_bindgen(method)]
    /// Append item to the tail of the menu.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menuappenditem)
    ///
    pub fn append(this:&Menu, item:&MenuItem);

    #[wasm_bindgen(method, js_name = insert)]
    /// Insert the item at ith position of the menu. The index is started from 0.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menuinsertitem-i)
    pub fn insert(this:&Menu, index:u16);

    #[wasm_bindgen(method, js_name = remove)]
    /// Remove item from the menu. This method requires you to keep the MenuItem outside the Menu.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menuremoveitem)
    pub fn remove(this:&Menu);

    #[wasm_bindgen(method, js_name = removeAt)]
    /// Remove the item form the menu by index.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menuremoveati)
    pub fn remove_at(this:&Menu, index:u16);

    #[wasm_bindgen(method, js_name = popup)]
    /// Popup the context menu at the anchor in (x, y) in current window.
    /// This method is only valid for contextmenu type.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menupopupx-y)
    pub fn popup(this:&Menu, x:i32, y:i32);

    #[wasm_bindgen(method, js_name=createMacBuiltin)]
    /// (Mac) Creates the builtin menus (App, Edit and Window) within the menubar on Mac.
    /// The items can be manipulated with the items property.
    /// The argument appname is used for the title of App menu.
    /// You can still use builtin menus with other menu items.
    /// i.e. append or insert items to the menu is still valid.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menucreatemacbuiltinappname-options-mac)
    ///
    pub fn create_mac_builtin(this:&Menu, app_name:&str);

    #[wasm_bindgen(method, js_name=createMacBuiltin)]
    /// (Mac) Creates the builtin menus (App, Edit and Window) within the menubar on Mac.
    /// The items can be manipulated with the items property.
    /// The argument appname is used for the title of App menu.
    /// You can still use builtin menus with other menu items.
    /// i.e. append or insert items to the menu is still valid.
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menucreatemacbuiltinappname-options-mac)
    ///
    pub fn create_mac_builtin_with_options(this:&Menu, app_name:&str, options:&MacOptions);
    

    // menu.insert(item, i)
    // menu.remove(item)
    // menu.removeAt(i)
    // menu.popup(x, y)
    // menu.createMacBuiltin(appname, [options]) 
    // Menu represents a menubar or a context menu. And MenuItem is item inside a menu. Please read the document of MenuItem for more details.
    
    // Synopsis
    // An example to create a context menu:
    // // Create an empty context menu
    // var menu = new nw.Menu();
    
    // // Add some items
    // menu.append(new nw.MenuItem({ label: 'Item A' }));
    // menu.append(new nw.MenuItem({ label: 'Item B' }));
    // menu.append(new nw.MenuItem({ type: 'separator' }));
    // menu.append(new nw.MenuItem({ label: 'Item C' }));
    
    // // Remove one item
    // menu.removeAt(1);
    
    // // Popup as context menu
    // menu.popup(10, 10);
    
    // // Iterate menu's items
    // for (var i = 0; i < menu.items.length; ++i) {
    //   console.log(menu.items[i]);
    // }
    // To create a menubar, usually you have to create a 2-level menu and assign it to win.menu. Here is the example of creating a menubar:
    // // Create an empty menubar
    // var menu = new nw.Menu({type: 'menubar'});
    
    // // Create a submenu as the 2nd level menu
    // var submenu = new nw.Menu();
    // submenu.append(new nw.MenuItem({ label: 'Item A' }));
    // submenu.append(new nw.MenuItem({ label: 'Item B' }));
    
    // // Create and append the 1st level menu to the menubar
    // menu.append(new nw.MenuItem({
    //   label: 'First Menu',
    //   submenu: submenu
    // }));
    
    // // Assign it to `window.menu` to get the menu displayed
    // nw.Window.get().menu = menu;
    // See Customize Menubar for detailed usages.
    
    // Using Menu With Page Navigation
    
    // Menus created in the page that can be navigated will not be functional after a reload or navigation. The reason is that the menus and even the web page will be garbage collected by JS engine after navigation to prevent memory leak. So it’s recommended to use menus in background page, which is existed for the life cycle of your app. See bg-script and main for how to execute scripts in the background page.
    
    // new Menu([option])
    // option {Object} Optional
    // type {String} Optional two types are accepted by this method: “menubar” or “contextmenu”. The value is set to “contextmenu” by default.
    // Create a Menu object.
    
    // menu.items
    // Get an array that contains all items of a menu. Each item is an instance of MenuItem. See MenuItem for detailed information.
    
    // menu.append(item)
    // item {MenuItem} the item to be appended to the tail of the menu
    // Append item to the tail of the menu.
    
    // menu.insert(item, i)
    // item {MenuItem} the item to be inserted into the menu
    // i {Integer} the index in the menu list to insert the the item
    // Insert the item at ith position of the menu. The index is started from 0.
    
    // menu.remove(item)
    // item {MenuItem} the item to be removed
    // Remove item from the menu. This method requires you to keep the MenuItem outside the Menu.
    
    // menu.removeAt(i)
    // i {Integer} the index of the item to be removed from the menu
    // Remove the ith item form the menu.
    
    // menu.popup(x, y)
    // x {Integer} the x position of the anchor
    // y {Integer} the y position of the anchor
    // Popup the context menu at the anchor in (x, y) in current window. This method is only valid for contextmenu type.
    
    // Usually you would listen to contextmenu event of DOM elements and manually popup the menu:
    
    // document.body.addEventListener('contextmenu', function(ev) { 
    //   ev.preventDefault();
    //   menu.popup(ev.x, ev.y);
    //   return false;
    // });
    // In this way, you can precisely choose which menu to show for different elements, and you can update menu elements just before popuping it.
    

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Options;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type MacOptions;
    

}

impl OptionsExt for Options{}
impl OptionsExt for MacOptions{}

impl MacOptions{
    /// do not populate the Edit menu
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menucreatemacbuiltinappname-options-mac)
    pub fn hide_edit(self, hide:bool)->Self{
        self.set("hideEdit", JsValue::from(hide))
    }
    /// do not populate the Window menu
    ///
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Menu/#menucreatemacbuiltinappname-options-mac)
    pub fn hide_window(self, hide:bool)->Self{
        self.set("hideWindow", JsValue::from(hide))
    }
}

pub enum Type {
    Menubar
}

impl From<Type> for Options{
    fn from(t: Type) -> Self {
        let options = Self::new();
        let options = match t{
            Type::Menubar=>{
                options.set("type", JsValue::from("menubar"))
            }
        };

        options
    }
}

