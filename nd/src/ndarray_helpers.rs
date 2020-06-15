// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use wasm_bindgen::JsValue;

/// Shorthand for js_sys::Error creation, wrapped in JsValue.
macro_rules! js_error {
  ($($arg:tt)*) => {{
    let res = $crate::js_sys::Error::new(&format!($($arg)*)).into();
    res
  }}
}

macro_rules! js_into {
  ($arg:expr, $type:ty) => {{
    let res: Result<$type, JsValue> = match $arg.into_serde() {
      Ok(ok) => Ok(ok),
      Err(e) => Err(js_error!("{}", &e.to_string())),
    };
    res
  }};
}

pub type JsReturn<T = JsValue> = Result<T, JsValue>;
