use std::sync::PoisonError;

use wasm_bindgen::JsValue;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error: {0}")]
    String(String),

    #[error("Error: {0}")]
    JsValue(String),

    #[error("Poison Error: {0}")]
    PoisonError(String)
}

impl From<String> for Error{
    fn from(v:String)->Self{
        Self::String(v)
    }
}
impl From<&str> for Error{
    fn from(v:&str)->Self{
        Self::String(v.to_string())
    }
}

impl From<JsValue> for Error{
    fn from(v:JsValue)->Self{
        Self::JsValue(format!("{:?}", v))
    }
}

impl<T> From<PoisonError<T>> for Error 
where T : std::fmt::Debug
{
    fn from(err: PoisonError<T>) -> Error {
        Error::PoisonError(format!("{:?}", err))
    }
}


impl From<Error> for String {
    fn from(err: Error) -> String {
        match err {
            Error::String(s) 
            | Error::PoisonError(s)
            | Error::JsValue(s)
                => String::from(s),
            //Error::RecvError => String::from(&format!("{}", err)),
        }
    }
}

impl From<Error> for JsValue{
    fn from(err: Error) -> JsValue {
        let s : String = err.into();
        JsValue::from_str(&s)
    }
}
