//!
//! Menu item control used in conjunction with the [`Menu`](crate::menu::Menu) interface.
//!
//! For usage example please refer to [nw_sys::menu](crate::menu)
//!

use crate::menu::Menu;
use crate::options::OptionsTrait;
use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    ///
    /// Interface for controlling menu items. For usage example please refer to [nw_sys::menu](crate::menu)
    ///

    #[wasm_bindgen(js_namespace=nw, js_name = MenuItem)]
    #[derive(Debug, Clone)]
    pub type MenuItem;

    #[wasm_bindgen(constructor, js_namespace=["nw"])]
    /// Create MenuItem
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#menuitem)
    ///
    pub fn new(options: &Options) -> MenuItem;

    #[wasm_bindgen(method, getter, js_name = label)]
    /// Get the label of a MenuItem
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemlabel)
    pub fn label(this: &MenuItem) -> String;

    #[wasm_bindgen(method, setter, js_name = label)]
    /// Set the label of a MenuItem
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemlabel)
    pub fn set_label(this: &MenuItem) -> String;

    #[wasm_bindgen(method, getter, js_name = type)]
    /// Get the type of a MenuItem
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemtype)
    pub fn get_type(this: &MenuItem) -> String;

    #[wasm_bindgen(method, getter, js_name = icon)]
    /// Get the icon of a MenuItem
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemicon)
    pub fn icon(this: &MenuItem) -> String;

    #[wasm_bindgen(method, setter, js_name = icon)]
    /// Set the icon of a MenuItem, it must a path to your icon file.
    /// It can be a relative path which points to an icon in your app,
    /// or an absolute path pointing to a file in user’s system.
    ///
    /// It has no effect on setting icon of a separator item.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemicon)
    pub fn set_icon(this: &MenuItem, icon: &str);

    #[wasm_bindgen(method, getter, js_name = iconIsTemplate)]
    /// (Mac) Get whether icon image is treated as "template".
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemiconistemplate-mac)
    pub fn icon_is_template(this: &MenuItem) -> bool;

    #[wasm_bindgen(method, setter, js_name = iconIsTemplate)]
    /// (Mac) Set whether icon image is treated as "template" (true by default).
    /// When the property is set to true the image is treated as "template"
    /// and the system automatically ensures proper styling according to the
    /// various states of the status item (e.g. dark menu, light menu, etc.).
    /// Template images should consist only of black and clear colours
    /// and can use the alpha channel in the image to adjust the opacity of
    /// black content. It has no effects on Linux and Windows.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemiconistemplate-mac)
    pub fn set_icon_is_template(this: &MenuItem, value: bool);

    #[wasm_bindgen(method, getter, js_name = tooltip)]
    /// (Mac) Get the tooltip of a MenuItem
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemtooltip-mac)
    pub fn tooltip(this: &MenuItem) -> String;

    #[wasm_bindgen(method, setter, js_name = tooltip)]
    /// (Mac) Set the tooltip of a MenuItem, it can only be plain text.
    /// A tooltip is short string that describes the menu item,
    /// it will show when you hover your mouse on the item.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemtooltip-mac)
    pub fn set_tooltip(this: &MenuItem, tooltip: &str);

    #[wasm_bindgen(method, getter, js_name = checked)]
    /// Get whether the MenuItem is checked.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemchecked)
    pub fn checked(this: &MenuItem) -> bool;

    #[wasm_bindgen(method, setter, js_name = checked)]
    /// Set whether the MenuItem is checked.
    /// Usually if a MenuItem is checked.
    /// There will be a mark on the left side of it.
    /// It’s used mostly to indicate whether an option is on.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemchecked)
    pub fn set_checked(this: &MenuItem, checked: bool);

    #[wasm_bindgen(method, getter, js_name = enabled)]
    /// Get whether the MenuItem is enabled.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemenabled)
    pub fn enabled(this: &MenuItem) -> bool;

    #[wasm_bindgen(method, setter, js_name = enabled)]
    /// Set whether the MenuItem is enabled.
    /// An disabled MenuItem will be greyed out and you will not be able to
    /// click on it.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemenabled)
    pub fn set_enabled(this: &MenuItem, enabled: bool);

    #[wasm_bindgen(method, getter, js_name = submenu)]
    /// Get the submenu of a MenuItem.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemsubmenu)
    pub fn submenu(this: &MenuItem) -> Option<Menu>;

    #[wasm_bindgen(method, setter, js_name = submenu)]
    /// Set the submenu of a MenuItem, the submenu must be a Menu object.
    /// You should set submenu in the option when creating the MenuItem.
    /// Changing it at runtime is slow on some platforms.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemsubmenu)
    pub fn set_submenu(this: &MenuItem, submenu: &Menu);

    /*
    #[wasm_bindgen(method, getter, js_name = enabled)]
    /// Get the submenu of a MenuItem.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemclick)
    pub fn click(this:&MenuItem)->Option<Function>;
    */

    #[wasm_bindgen(method, setter, js_name = click)]
    /// Set the submenu of a MenuItem, the submenu must be a Menu object.
    /// You should set submenu in the option when creating the MenuItem.
    /// Changing it at runtime is slow on some platforms.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemclick)
    pub fn set_click(this: &MenuItem, callback: &Function);

    #[wasm_bindgen(method, getter, js_name = key)]
    /// A single character string to specify the shortcut key for the menu item.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemkey)
    pub fn key(this: &MenuItem) -> String;

    #[wasm_bindgen(method, setter, js_name = key)]
    /// Set shortcut key
    ///
    /// Valid Keys for All Platforms
    /// Alphabet: a-z
    /// Digits: 0-9
    /// Other keys on main area: [ , ] , ' , , , . , / , ` , - , = , \ , ' , ; , Tab
    /// Esc
    /// Down , Up , Left , Right
    /// W3C DOM Level 3 KeyboardEvent Key Values: KeyA (same as A),
    /// Escape (same as Esc), F1, ArrowDown (same as Down) etc.
    /// Special Keys for Mac Only
    ///
    /// On Mac, you can also use special keys as shortcut key with
    /// String.fromCharCode(specialKey).
    ///
    /// Here are some useful keys:
    ///
    /// 28: Left (←)
    /// 29: Right (→)
    /// 30: Up (↑)
    /// 31: Down (↓)
    /// 27: Escape (⎋)
    /// 11: PageUp (⇞)
    /// 12: PageDown (⇟)
    /// For full list of special keys supported on Mac,
    /// see [NSMenuItem.keyEquivalent](https://developer.apple.com/library/mac/documentation/Cocoa/Reference/ApplicationKit/Classes/NSMenuItem_Class/#//apple_ref/occ/instp/NSMenuItem/keyEquivalent)
    /// and [NSEvent: Function-Key Unicodes](https://developer.apple.com/library/mac/documentation/Cocoa/Reference/ApplicationKit/Classes/NSEvent_Class/index.html#//apple_ref/doc/constant_group/Function_Key_Unicodes).
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemkey)
    pub fn set_key(this: &MenuItem, key: &str);

    #[wasm_bindgen(method, getter, js_name = modifiers)]
    /// Get modifier keys
    /// A string to specify the modifier keys for the shortcut of the menu item.
    /// It should be the concatenation of the following strings:
    /// cmd / command / super, shift, ctrl, alt. e.g. "cmd+shift+alt".
    /// cmd represents different keys on
    /// all platforms: Windows key (Windows) on Windows and Linux,
    /// Apple key (⌘) on Mac. super and command are aliases of cmd.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemmodifiers)
    pub fn modifiers(this: &MenuItem) -> String;

    #[wasm_bindgen(method, setter, js_name = modifiers)]
    /// Set modifier keys
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#itemmodifiers)
    pub fn set_modifiers(this: &MenuItem, key: &str);

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Options;

}

impl OptionsTrait for Options {}

impl Options {
    /// Type of MenuItem
    ///
    /// Three types are accepted: normal, checkbox, separator
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn set_type(self, t: Type) -> Self {
        self.set("type", t.into())
    }

    /// Label for normal item or checkbox
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn label(self, label: &str) -> Self {
        self.set("label", JsValue::from(label))
    }

    /// Icon for normal item or checkbox
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn icon(self, icon: &str) -> Self {
        self.set("icon", JsValue::from(icon))
    }

    /// Tooltip for normal item or checkbox
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn tooltip(self, tooltip: &str) -> Self {
        self.set("tooltip", JsValue::from(tooltip))
    }

    /// The callback function when item is triggered by mouse click or keyboard shortcut
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn click(self, callback: &Function) -> Self {
        self.set("click", JsValue::from(callback))
    }

    /// Whether the item is enabled or disabled. It’s set to true by default.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn enabled(self, enabled: bool) -> Self {
        self.set("enabled", JsValue::from(enabled))
    }

    /// Whether the checkbox is checked or not. It’s set to false by default.
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn checked(self, checked: bool) -> Self {
        self.set("checked", JsValue::from(checked))
    }

    /// A submenu
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn submenu(self, submenu: &Menu) -> Self {
        self.set("submenu", JsValue::from(submenu))
    }

    /// The key of the shortcut
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn key(self, key: &str) -> Self {
        self.set("key", JsValue::from(key))
    }

    /// The modifiers of the shortcut
    ///
    /// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
    pub fn modifiers(self, modifiers: &str) -> Self {
        self.set("modifiers", JsValue::from(modifiers))
    }
}

/// Type of menu item
///
/// Three types are accepted: Normal, Checkbox, Separator
///
/// ⧉ [NWJS Documentation](https://docs.nwjs.io/en/latest/References/MenuItem/#new-menuitemoption)
///
pub enum Type {
    Separator,
    Normal,
    Checkbox,
}

impl From<Type> for Options {
    fn from(t: Type) -> Self {
        match t {
            Type::Separator => Options::new().set_type(Type::Separator),
            Type::Normal => Options::new().set_type(Type::Normal),
            Type::Checkbox => Options::new().set_type(Type::Checkbox),
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::Separator => "Separator",
            Type::Normal => "Normal",
            Type::Checkbox => "Checkbox",
        }
        .to_string()
    }
}

impl From<Type> for JsValue {
    fn from(t: Type) -> Self {
        JsValue::from(t.to_string().to_lowercase())
    }
}
