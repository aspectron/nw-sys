use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern "C" {

    // Menu
    // Menu
    // Synopsis
    // new Menu([option])
    // menu.items
    // menu.append(item)
    // menu.insert(item, i)
    // menu.remove(item)
    // menu.removeAt(i)
    // menu.popup(x, y)
    // menu.createMacBuiltin(appname, [options]) (Mac)
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
    
    // menu.createMacBuiltin(appname, [options]) (Mac)
    // appname {String} The application name
    // options {Object} Optional
    // hideEdit {Boolean} Optional do not populate the Edit menu
    // hideWindow {Boolean} Optional do not populate the Window menu
    // Creates the builtin menus (App, Edit and Window) within the menubar on Mac. The items can be manipulated with the items property. The argument appname is used for the title of App menu.
    
    // You can still use builtin menus with other menu items. i.e. append or insert items to the menu is still valid.
    
    // See also Customize Menubar for detailed usage.

}
