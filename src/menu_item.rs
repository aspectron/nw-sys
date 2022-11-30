use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {
    // MenuItem
    // MenuItem
    // Synopsis
    // new MenuItem(option)
    // item.type
    // item.label
    // item.icon
    // item.iconIsTemplate (Mac)
    // item.tooltip (Mac)
    // item.checked
    // item.enabled
    // item.submenu
    // item.click
    // item.key
    // Valid Keys for All Platforms
    // Special Keys for Mac Only
    // item.modifiers
    // Event: click
    // MenuItem represents an item in a menu. A MenuItem can be a separator or a normal item which has label and icon or a checkbox. It can response to mouse click or keyboard shortcut.
    
    // Synopsis
    // var item;
    
    // // Create a separator
    // item = new nw.MenuItem({ type: 'separator' });
    
    // // Create a normal item with label and icon
    // item = new nw.MenuItem({
    //   type: "normal", 
    //   label: "I'm a menu item",
    //   icon: "img/icon.png"
    // });
    
    // // Or you can omit the 'type' field for normal items
    // item = new nw.MenuItem({ label: 'Simple item' });
    
    // // Bind a callback to item
    // item = new nw.MenuItem({
    //   label: "Click me",
    //   click: function() {
    //     console.log("I'm clicked");
    //   },
    //   key: "s",
    //   modifiers: "ctrl+alt",
    // });
    
    // // You can have submenu!
    // var submenu = new nw.Menu();
    // submenu.append(new nw.MenuItem({ label: 'Item 1' }));
    // submenu.append(new nw.MenuItem({ label: 'Item 2' }));
    // submenu.append(new nw.MenuItem({ label: 'Item 3' }));
    // item.submenu = submenu;
    
    // // And everything can be changed at runtime
    // item.label = 'New label';
    // item.click = function() { console.log('New click callback'); };
    // new MenuItem(option)
    // option {Object} an object contains initial settings for the MenuItem
    // label {String} Optional label for normal item or checkbox
    // icon {String} Optional icon for normal item or checkbox
    // tooltip {String} Optional tooltip for normal item or checkbox
    // type {String} Optional the type of the item. Three types are accepted: normal, checkbox, separator
    // click {Function} Optional the callback function when item is triggered by mouse click or keyboard shortcut
    // enabled {Boolean} Optional whether the item is enabled or disabled. It’s set to true by default.
    // checked {Boolean} Optional whether the checkbox is checked or not. It’s set to false by default.
    // submenu {Menu} Optional a submenu
    // key {String} Optional the key of the shortcut
    // modifiers {String} Optional the modifiers of the shortcut
    // Every field has its own property in the MenuItem, see documentation of each property for details.
    
    // MenuItem is inherited from EventEmitter. You can use on to listen to the events.
    
    // item.type
    // Get the type of a MenuItem, it can be separator, checkbox and normal.
    
    // Note
    
    // The type can be set only when you create it. It cannot be changed at runtime.
    
    // item.label
    // Get or set the label of a MenuItem, it can only be plain text for now.
    
    // item.icon
    // Get or set the icon of a MenuItem, it must a path to your icon file. It can be a relative path which points to an icon in your app, or an absolute path pointing to a file in user’s system.
    
    // It has no effect on setting icon of a separator item.
    
    // item.iconIsTemplate (Mac)
    // Get or set whether icon image is treated as “template” (true by default). When the property is set to true the image is treated as “template” and the system automatically ensures proper styling according to the various states of the status item (e.g. dark menu, light menu, etc.). Template images should consist only of black and clear colours and can use the alpha channel in the image to adjust the opacity of black content. It has no effects on Linux and Windows.
    
    // item.tooltip (Mac)
    // Get or set the tooltip of a MenuItem, it can only be plain text. A tooltip is short string that describes the menu item, it will show when you hover your mouse on the item.
    
    // item.checked
    // Get or set whether the MenuItem is checked. Usually if a MenuItem is checked. There will be a mark on the left side of it. It’s used mostly to indicate whether an option is on.
    
    // item.enabled
    // Get or set whether the MenuItem is enabled. An disabled MenuItem will be greyed out and you will not be able to click on it.
    
    // item.submenu
    // Get or set the submenu of a MenuItem, the submenu must be a Menu object. You should set submenu in the option when creating the MenuItem. Changing it at runtime is slow on some platforms.
    
    // item.click
    // Get or set the click callback of a MenuItem, the click must be a valid function. It will be called when users activate the item.
    
    // item.key
    // A single character string to specify the shortcut key for the menu item.
    
    // Valid Keys for All Platforms
    // Alphabet: a-z
    // Digits: 0-9
    // Other keys on main area: [ , ] , ' , , , . , / , ` , - , = , \ , ' , ; , Tab
    // Esc
    // Down , Up , Left , Right
    // W3C DOM Level 3 KeyboardEvent Key Values: KeyA (same as A), Escape (same as Esc), F1, ArrowDown (same as Down) etc.
    // Special Keys for Mac Only
    // On Mac, you can also use special keys as shortcut key with String.fromCharCode(specialKey). Here are some useful keys:
    
    // 28: Left (←)
    // 29: Right (→)
    // 30: Up (↑)
    // 31: Down (↓)
    // 27: Escape (⎋)
    // 11: PageUp (⇞)
    // 12: PageDown (⇟)
    // For full list of special keys supported on Mac, see NSMenuItem.keyEquivalent and NSEvent: Function-Key Unicodes.
    
    // item.modifiers
    // A string to specify the modifier keys for the shortcut of the menu item. It should be the concatenation of the following strings: cmd / command / super, shift, ctrl, alt. e.g. "cmd+shift+alt".
    
    // cmd represents different keys on all platforms: Windows key (Windows) on Windows and Linux, Apple key (⌘) on Mac. super and command are aliases of cmd.
    
    // Event: click
    // Emitted when user activates the menu item.
}
