//! # Synopsis
//! ```rust
//! 
//! // Create basic notification
//! let options = notifications::Options::new()
//!     .title("Title text")
//!     .icon_url("/resources/icons/tray-icon@2x.png")
//!     .set_type(nw_sys::chrome::notifications::TemplateType::Basic)
//!     .message("Message Text")
//!     .context_message("Context Message");
//! 
//! notifications::create(None, &options, None);
//! 
//! // Create notification with buttons
//! let button1 = notifications::Button::new()
//!     .title("Button A")
//!     .icon_url("/resources/icons/tray-icon@2x.png");
//! 
//! let button2 = notifications::Button::new()
//!     .title("Button B")
//!     .icon_url("/resources/icons/tray-icon@2x.png");
//! 
//! let options = notifications::Options::new()
//!     .title("Title text")
//!     .icon_url("/resources/icons/tray-icon@2x.png")
//!     .set_type(nw_sys::chrome::notifications::TemplateType::Basic)
//!     .message("Message Text")
//!     .buttons(vec![button1, button2]);
//! 
//! notifications::create(None, &options, None);
//! 
//! // Create image notification
//! let options = notifications::Options::new()
//!     .title("Title text")
//!     .icon_url("/resources/icons/tray-icon@2x.png")
//!     .set_type(nw_sys::chrome::notifications::TemplateType::Image)
//!     .message("Message Text")
//!     .image_url("/resources/setup/document.png");
//! 
//! notifications::create(None, &options, None);
//! 
//! // Create notification with items
//! let item1 = notifications::Item::new()
//!     .title("Item A")
//!     .message("Mesage A");
//! let item2 = notifications::Item::new()
//!     .title("Item B")
//!     .message("Mesage B");
//! 
//! let options = notifications::Options::new()
//!     .title("Title text")
//!     .icon_url("/resources/icons/tray-icon@2x.png")
//!     .set_type(nw_sys::chrome::notifications::TemplateType::List)
//!     .message("Message Text")
//!     .items(vec![item1, item2]);
//! 
//! notifications::create(None, &options, None);
//!
//! // Create notification with progress
//! let options = notifications::Options::new()
//!     .title("Title text")
//!     .icon_url("/resources/icons/tray-icon@2x.png")
//!     .set_type(nw_sys::chrome::notifications::TemplateType::Progress)
//!     .message("Mesage text")
//!     .progress(50);
//! notifications::create(None, &options, None);
//!  
//! ```
//! 

use wasm_bindgen::prelude::*;
use js_sys::{Object, Array, Function};
use crate::options::OptionsExt;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=["chrome", "notifications"], js_name="create")]
    fn create_impl(options:&Options);

    #[wasm_bindgen(js_namespace=["chrome", "notifications"], js_name="create")]
    fn create_with_id_impl(id:&str, options:&Options);

    #[wasm_bindgen(js_namespace=["chrome", "notifications"], js_name="create")]
    fn create_with_callback_impl(options:&Options, callback:&Function);

    #[wasm_bindgen(js_namespace=["chrome", "notifications"], js_name="create")]
    fn create_with_id_and_callback_impl(id:&str, options:&Options, callback:&Function);

    #[wasm_bindgen(js_namespace=["chrome", "notifications"], js_name="clear")]
    fn clear_impl(id:&str);

    #[wasm_bindgen(js_namespace=["chrome", "notifications"], js_name="clear")]
    fn clear_with_callback_impl(id:&str, callback:&Function);

    #[wasm_bindgen(js_namespace=["chrome", "notifications"], js_name="getAll")]
    /// Retrieves all the notifications of this app or extension.
    ///
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#method-getAll)
    ///
    pub fn get_all(callback:&Function);

    #[wasm_bindgen(js_namespace=["chrome", "notifications"], js_name="getPermissionLevel")]
    /// Retrieves whether the user has enabled notifications from this app or extension.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#method-getPermissionLevel)
    ///
    pub fn get_permission_level(callback:&Function);

    #[wasm_bindgen(js_namespace=["chrome", "notifications"], js_name="update")]
    fn update_impl(id:&str, options:&Options);

    #[wasm_bindgen(js_namespace=["chrome", "notifications"], js_name="update")]
    fn update_with_callback_impl(id:&str, options:&Options, callback:&Function);

    #[wasm_bindgen(js_namespace=["chrome", "notifications", "onButtonClicked"], js_name="addListener")]
    /// The user pressed a button in the notification.
    /// 
    /// The `callback` parameter looks like:
    /// (notification_id: String, button_index: u16) => ()
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#event-onButtonClicked)
    pub fn on_button_clicked(callback:&Function);

    #[wasm_bindgen(js_namespace=["chrome", "notifications", "onClicked"], js_name="addListener")]
    /// The user clicked in a non-button area of the notification.
    /// 
    /// The `callback` parameter looks like:
    /// (notification_id: String) => ()
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#event-onClicked)
    pub fn on_clicked(callback:&Function);

    #[wasm_bindgen(js_namespace=["chrome", "notifications", "onClosed"], js_name="addListener")]
    /// The notification closed, either by the system or by user action.
    /// 
    /// The `callback` parameter looks like:
    /// (notification_id: String, by_user:bool) => ()
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#event-onClosed)
    pub fn on_closed(callback:&Function);

    #[wasm_bindgen(js_namespace=["chrome", "notifications", "onPermissionLevelChanged"], js_name="addListener")]
    /// The user changes the permission level.
    /// As of Chrome 47, only ChromeOS has UI that dispatches this event.
    /// 
    /// The `callback` parameter looks like:
    /// (level: JsValue ) => ()
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#event-onPermissionLevelChanged)
    pub fn on_permission_level_changed(callback:&Function);

    /// Notification Options
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#type-NotificationOptions)
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Options;

    /// Notification Button
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#type-NotificationButton)
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Button;

    /// Notification Item
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#type-NotificationItem)
    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type Item;
}

impl OptionsExt for Options{}
impl OptionsExt for Button{}
impl OptionsExt for Item{}

/// Notification template type
pub enum TemplateType{
    Basic,
    Image,
    List,
    Progress
}

impl ToString for TemplateType{
    fn to_string(&self) -> String {
        match self {
            Self::Basic=>"Basic".to_string(),
            Self::Image=>"Image".to_string(),
            Self::List=>"List".to_string(),
            Self::Progress=>"Progress".to_string()
        }
    }
}

impl From<TemplateType> for JsValue{
    fn from(value: TemplateType) -> Self {
        Self::from(value.to_string().to_lowercase())
    }
}

impl Button{
    /// Button icons not visible for Mac OS X users.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationButton-iconUrl)
    pub fn icon_url(self, url:&str)->Self{
        self.set("iconUrl", JsValue::from(url))
    }

    /// Button text
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationButton-title)
    pub fn title(self, title:&str)->Self{
        self.set("title", JsValue::from(title))
    }
}

impl Item{
    /// Additional details about this item.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationItem-message)
    pub fn message(self, message:&str)->Self{
        self.set("message", JsValue::from(message))
    }

    /// Title of one item of a list notification.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationItem-title)
    pub fn title(self, title:&str)->Self{
        self.set("title", JsValue::from(title))
    }
}

impl Options{
    /// 
    /// *Note* : The app icon mask is not visible for Mac OS X users.
    /// 
    /// A URL to the app icon mask.
    /// URLs have the same restrictions as iconUrl.
    /// The app icon mask should be in alpha channel, as only the alpha channel 
    /// of the image will be considered.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-appIconMaskUrl)
    pub fn app_icon_mask_url(self, url:&str)->Self{
        self.set("appIconMaskUrl", JsValue::from(url))
    }

    /// Text and icons for up to two notification action buttons.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-buttons)
    pub fn buttons(self, buttons:Vec<Button>)->Self{
        let array = Array::new();
        for button in buttons{
            array.push(&JsValue::from(button));
        }
        self.set("buttons", JsValue::from(array))
    }

    /// Alternate notification content with a lower-weight font.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-contextMessage)
    pub fn context_message(self, context_message:&str)->Self{
        self.set("contextMessage", JsValue::from(context_message))
    }

    /// A timestamp associated with the notification, in milliseconds past the epoch (e.g. Date.now() + n).
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-eventTime)
    pub fn event_time(self, event_time:u32)->Self{
        self.set("eventTime", JsValue::from(event_time))
    }

    /// A URL to the sender's avatar, app icon, or a thumbnail for image notifications.
    ///
    /// URLs can be a data URL, a blob URL, or a URL relative to a 
    /// resource within this extension's .crx file Required for [notifications.create()](self::create) method.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-iconUrl)
    pub fn icon_url(self, icon_url:&str)->Self{
        self.set("iconUrl", JsValue::from(icon_url))
    }

    /// The image is not visible for Mac OS X users.
    /// 
    /// A URL to the image thumbnail for image-type notifications. 
    /// URLs have the same restrictions as [iconUrl](self::Options#method.icon_url).
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-imageUrl)
    pub fn image_url(self, image_url:&str)->Self{
        self.set("imageUrl", JsValue::from(image_url))
    }


    /// Items for multi-item notifications.
    /// Users on Mac OS X only see the first item.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-items)
    pub fn items(self, items:Vec<Item>)->Self{
        let array = Array::new();
        for item in items{
            array.push(&JsValue::from(item));
        }
        self.set("items", JsValue::from(array))
    }

    /// Main notification content.
    /// Required for [notifications.create](self::create) method.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-message)
    pub fn message(self, message:&str)->Self{
        self.set("message", JsValue::from(message))
    }

    /// Priority ranges from -2 to 2. -2 is lowest priority. 2 is highest.
    /// Zero is default.
    /// On platforms that don't support a notification center 
    /// (Windows, Linux & Mac), -2 and -1 result in an error as notifications with 
    /// those priorities will not be shown at all.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-priority)
    pub fn priority(self, priority:i8)->Self{
        self.set("priority", JsValue::from(priority))
    }

    /// Current progress ranges from 0 to 100.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-progress)
    pub fn progress(self, progress:u8)->Self{
        self.set("progress", JsValue::from(progress))
    }

    /// Indicates that the notification should remain visible on screen until 
    /// the user activates or dismisses the notification. This defaults to false.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-requireInteraction)
    pub fn require_interaction(self, require_interaction:bool)->Self{
        self.set("requireInteraction", JsValue::from(require_interaction))
    }

    /// Indicates that no sounds or vibrations should be made when the 
    /// notification is being shown. This defaults to false.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-silent)
    pub fn silent(self, silent:bool)->Self{
        self.set("silent", JsValue::from(silent))
    }

    /// Title of the notification (e.g. sender name for email).
    /// Required for [notifications.create](self::create) method.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-title)
    pub fn title(self, title:&str)->Self{
        self.set("title", JsValue::from(title))
    }

    /// Which type of notification to display. 
    /// 
    /// Required for [notifications.create](self::create) method.
    /// 
    /// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#property-NotificationOptions-type)
    pub fn set_type(self, template_type:TemplateType)->Self{
        self.set("type", template_type.into())
    }


}



/// Creates and displays a notification.
/// 
/// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#method-create)
/// 
pub fn create(id:Option<String>, options:&Options, callback:Option<&Function>){
    if id.is_some() && callback.is_some(){
        create_with_id_and_callback_impl(&id.unwrap(), options, callback.unwrap());
    }else if id.is_some(){
        create_with_id_impl(&id.unwrap(), options);
    }else if callback.is_some(){
        create_with_callback_impl(options, callback.unwrap());
    }else{
        create_impl(options);
    }
}

/// Clears the specified notification.
/// 
/// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#method-clear)
/// 
pub fn clear(id:&str, callback:Option<&Function>){
    if let Some(callback) = callback{
        clear_with_callback_impl(id, callback);
    }else{
        clear_impl(id);
    }
}

/// Updates an existing notification.
/// 
/// [Chrome Doc](https://developer.chrome.com/docs/extensions/reference/notifications/#method-update)
/// 
pub fn update(id:&str, options:&Options, callback:Option<&Function>){
    if let Some(callback) = callback{
        update_with_callback_impl(id, options, callback);
    }else{
        update_impl(id, options);
    }
}

