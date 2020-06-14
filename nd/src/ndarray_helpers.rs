// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

/// Shorthand for js_sys::Error creation, wrapped in JsValue.
macro_rules! js_error {
    ($($arg:tt)*) => {{
        let res = $crate::js_sys::Error::new(&format!($($arg)*)).into();
        res
    }}
}
