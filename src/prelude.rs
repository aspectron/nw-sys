//!
//! nw-sys prelude
//!

pub use crate::error;
pub use crate::options::OptionsTrait;
pub use crate::result;
pub use std::sync::{Arc, Mutex};

pub use crate::menu_item::Type as MenuItemType;
pub use crate::nw::{is_nw, try_nw};
pub use crate::tray::Options as TrayOptions;
pub use crate::utils::window;
pub use crate::window;
pub use crate::Clipboard;
pub use crate::Menu;
pub use crate::MenuItem;
pub use crate::Nw;
pub use crate::Shortcut;
pub use crate::Tray;
pub use crate::Window;
