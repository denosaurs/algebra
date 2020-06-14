// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;

//
// array info
//
#[wasm_bindgen(js_name = ndarray_f32_ndim)]
pub fn ndim(ndarray: JsValue) -> usize {
  let mat: ArrayD<f32> = ndarray
    .into_serde()
    .expect_throw("Self(ndarrayf32) is not valid.");
  mat.ndim()
}

#[wasm_bindgen(js_name = ndarray_f32_len)]
pub fn len(ndarray: JsValue) -> usize {
  let mat: ArrayD<f32> = ndarray
    .into_serde()
    .expect_throw("Self(ndarrayf32) is not valid.");
  mat.len()
}

#[wasm_bindgen(js_name = ndarray_f32_shape)]
pub fn shape(ndarray: JsValue) -> Vec<usize> {
  let mat: ArrayD<f32> = ndarray
    .into_serde()
    .expect_throw("Self(ndarrayf32) is not valid.");
  mat.shape().to_vec()
}

#[wasm_bindgen(js_name = ndarray_f32_len_of)]
pub fn len_of(ndarray: JsValue, axis: usize) -> usize {
  let mat: ArrayD<f32> = ndarray
    .into_serde()
    .expect_throw("Self(ndarrayf32) is not valid.");
  mat.len_of(Axis(axis))
}

#[wasm_bindgen(js_name = ndarray_f32_strides)]
pub fn strides(ndarray: JsValue) -> Vec<isize> {
  let mat: ArrayD<f32> = ndarray
    .into_serde()
    .expect_throw("Self(ndarrayf32) is not valid.");
  mat.strides().to_vec()
}

#[wasm_bindgen(js_name = ndarray_f32_is_empty)]
pub fn is_empty(ndarray: JsValue) -> bool {
  let mat: ArrayD<f32> = ndarray
    .into_serde()
    .expect_throw("Self(ndarrayf32) is not valid.");
  mat.is_empty()
}
