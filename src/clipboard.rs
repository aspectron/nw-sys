use wasm_bindgen::prelude::*;
use js_sys::{Object, Array};
//use workflow_log::log_info;
use crate::options::OptionsExt;
use crate::result::Result;


#[wasm_bindgen]
extern "C" {

    
    /// # Synopsis
    /// ```
    /// // get the system clipboard
    /// let clipboard = nw::Clipboard::get();
    /// ```
    #[wasm_bindgen(js_namespace=nw, js_name = Clipboard)]
    #[derive(Debug, Clone)]
    pub type Clipboard;
    
    #[wasm_bindgen(static_method_of=Clipboard, js_namespace=["nw"], js_name = get)]
    /// Returns the `Clipboard` object
    /// 
    /// **Note:**
    /// The Selection Clipboard in X11 is not supported.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipboardget)
    ///
    pub fn get()->Clipboard;

    #[wasm_bindgen(method, js_name = set)]
    /// Write data of type to the clipboard.
    /// This method will clear the clipboard and replace with the given data.
    /// Hence another call to this method will overwrite with the new one.
    /// To write multiple types of data to clipboard simultaneously,
    /// you will need to use [clip.set(clipboardDataList)](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetclipboarddatalist) below.
    /// 
    /// - data: the data to write to the clipboard
    ///  
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetdata-type-raw)
    ///
    pub fn write(this:&Clipboard, data:&str);

    #[wasm_bindgen(method, js_name = set)]
    /// Write data of type to the clipboard.
    /// This method will clear the clipboard and replace with the given data.
    /// Hence another call to this method will overwrite with the new one.
    /// To write multiple types of data to clipboard simultaneously,
    /// you will need to use [clip.set(clipboardDataList)](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetclipboarddatalist) below.
    /// 
    /// - data: the data to write to the clipboard
    /// - data_type: the type of the data. Support text, png, jpeg, html and rtf. By default, type is set to "text".
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetdata-type-raw)
    ///
    pub fn write_with_data_type(this:&Clipboard, data:&str, data_type:&str);

    #[wasm_bindgen(method, js_name = set)]
    /// Write data of type to the clipboard.
    /// This method will clear the clipboard and replace with the given data.
    /// Hence another call to this method will overwrite with the new one.
    /// To write multiple types of data to clipboard simultaneously,
    /// you will need to use [clip.set(clipboardDataList)](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetclipboarddatalist) below.
    /// 
    /// - data: the data to write to the clipboard
    /// - data_type: the type of the data. Support text, png, jpeg, html and rtf. By default, type is set to "text".
    /// - raw: requiring raw image data. This option is only valid if type is png or jpeg. By default, raw is set to false.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetdata-type-raw)
    ///
    pub fn write_with_data_type_and_raw(this:&Clipboard, data:&str, data_type:&str, raw:bool);

    #[wasm_bindgen(method, js_name = set)]
    /// Write data of type to the clipboard.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetclipboarddata)
    ///
    pub fn write_data(this:&Clipboard, data:DataWrite);

    #[wasm_bindgen(method, js_name = set)]
    /// Write data of type to the clipboard.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetclipboarddata)
    ///
    fn write_data_array_impl(this:&Clipboard, data:Array);

    #[wasm_bindgen(method, js_name = get)]
    /// Get the data
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipgettype-raw)
    ///
    pub fn read(this:&Clipboard)->String;

    #[wasm_bindgen(method, js_name = get)]
    /// Get the data of type
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipgettype-raw)
    ///
    pub fn read_with_data_type(this:&Clipboard, data_type:&str)->String;

    #[wasm_bindgen(method, js_name = get)]
    /// Get the data of type and as raw
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipgettype-raw)
    ///
    pub fn read_with_data_type_and_raw(this:&Clipboard, data_type:&str, raw:bool)->String;

    #[wasm_bindgen(method, js_name = get)]
    /// Get the data
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipgetclipboarddata)
    ///
    pub fn read_data(this:&Clipboard, data:DataRead)->String;

    #[wasm_bindgen(method, js_name = get)]
    fn read_data_array_impl(this:&Clipboard, data:Array)->Array;

    #[wasm_bindgen(method, js_name = readAvailableTypes)]
    fn read_available_types_impl(this:&Clipboard)->Array;

    #[wasm_bindgen(method)]
    /// Clear the clipboard.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipclear)
    /// 
    pub fn clear(this:&Clipboard);

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type DataWrite;

    #[wasm_bindgen(extends = Object)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub type DataRead;
}


impl OptionsExt for DataWrite{
    fn initialize(self)->Self{
        self.data_type("text")
            .raw(false)
    }
}
impl OptionsExt for DataRead{}

impl DataWrite{
    /// The data to write to the clipboard
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetdata-type-raw)
    pub fn data(self, data:&str)->Self{
        self.set("data", JsValue::from(data))
    }

    /// The type of the data.
    /// Support text, png, jpeg, html and rtf.
    /// 
    /// By default, type is set to "text".
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetdata-type-raw)
    pub fn data_type(self, data_type:&str)->Self{
        self.set("type", JsValue::from(data_type))
    }

    /// Requiring raw image data.
    /// This option is only valid if type is png or jpeg.
    /// 
    /// By default, raw is set to false.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetdata-type-raw)
    pub fn raw(self, raw:bool)->Self{
        self.set("raw", JsValue::from(raw))
    }
}

impl DataRead{
    /// The type of the data.
    /// Support text, png, jpeg, html and rtf.
    /// 
    /// By default, type is set to "text".
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetdata-type-raw)
    pub fn data_type(self, data_type:&str)->Self{
        self.set("type", JsValue::from(data_type))
    }

    /// Requiring raw image data.
    /// This option is only valid if type is png or jpeg.
    /// 
    /// By default, raw is set to false.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipsetdata-type-raw)
    pub fn raw(self, raw:bool)->Self{
        self.set("raw", JsValue::from(raw))
    }
}

impl From<(String, Option<bool>)> for DataRead{
    fn from(info: (String, Option<bool>)) -> Self {
        let option = Self::new().data_type(&info.0);
        if let Some(raw) = info.1{
            option.raw(raw)
        }else{
            option
        }
    }
}

impl Clipboard{
    pub fn write_data_array(&self, list:Vec<DataWrite>){
        let data_array = Array::new();
        for d in list{
            data_array.push(&JsValue::from(d));
        }
        self.write_data_array_impl(data_array);
    }

    /// Get the data
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipgetclipboarddatalist)
    ///
    pub fn read_data_array(&self, list:Vec<DataRead>)->Result<Vec<Option<String>>>{
        let data_array = Array::new();
        for d in list{
            data_array.push(&JsValue::from(d));
        }
        let array = self.read_data_array_impl(data_array);
        let mut result = Vec::new();
        for index in 0..array.length(){
            let data = array.get(index);
            let data = js_sys::Reflect::get(&data, &JsValue::from("data"))?;
            result.push(data.as_string());
        }

        Ok(result)
    }

    /// Returns list of available types of data in clipboard currently.
    /// 
    /// ### Each item is one of following types:
    /// - text: plain text. Can be read by `clip.read('text')`.
    /// - html: HTML text. Can be read by `clip.read('html')`.
    /// - rtf: RTF (Rich Text Format). Can be read by `clip.read('rtf')`.
    /// - png: PNG image. Can be read by `clip.read('png')`.
    /// - jpeg: JPEG image. Can be read by `clip.read('jpeg')`.
    /// 
    /// You can use the returned list as a suggestion to get the right data from clipboard.
    /// 
    /// [NWJS Documentation](https://docs.nwjs.io/en/latest/References/Clipboard/#clipreadavailabletypes)
    ///
    pub fn read_available_types(&self)->Vec<String>{
        let array = self.read_available_types_impl();
        let mut result = Vec::new();
        for index in 0..array.length(){
            if let Some(v) = array.get(index).as_string(){
                result.push(v);
            }
        }

        result
    }
}
