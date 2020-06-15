// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use crate::ndarray_helpers::JsReturn;
use ndarray::prelude::*;
use wasm_bindgen::prelude::*;

//
// array info
//
#[wasm_bindgen(js_name = ndarray_f32_ndim)]
pub fn ndim(ndarray: JsValue) -> JsReturn<usize> {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(mat.ndim())
}

#[wasm_bindgen(js_name = ndarray_f32_len)]
pub fn len(ndarray: JsValue) -> JsReturn<usize> {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(mat.len())
}

#[wasm_bindgen(js_name = ndarray_f32_shape)]
pub fn shape(ndarray: JsValue) -> JsReturn<Vec<usize>> {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(mat.shape().to_vec())
}

#[wasm_bindgen(js_name = ndarray_f32_len_of)]
pub fn len_of(ndarray: JsValue, axis: usize) -> JsReturn<usize> {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(mat.len_of(Axis(axis)))
}

#[wasm_bindgen(js_name = ndarray_f32_strides)]
pub fn strides(ndarray: JsValue) -> JsReturn<Vec<isize>> {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(mat.strides().to_vec())
}

#[wasm_bindgen(js_name = ndarray_f32_is_empty)]
pub fn is_empty(ndarray: JsValue) -> JsReturn<bool> {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(mat.is_empty())
}
