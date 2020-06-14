// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = ndarray_f32_dot)]
pub fn dot(lhs: JsValue, rhs: JsValue) -> Result<JsValue, JsValue> {
  let lhs: Array2<f32> = match lhs.into_serde() {
    Ok(lhs) => lhs,
    Err(e) => return Err(js_error!("{}", &e.to_string())),
  };
  let rhs: Array2<f32> = rhs
    .into_serde()
    .expect_throw("Self(ndarrayf32) is not valid.");
  Ok(JsValue::from_serde(&lhs.dot(&rhs)).unwrap())
}
