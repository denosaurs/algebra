// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;

use crate::ndarray_helpers::JsReturn;
use ndarray::Ix;

//
// array creation
//
#[wasm_bindgen(js_name = ndarray_f32_range)]
pub fn range(start: f32, end: f32, step: f32) -> JsValue {
  let mat: Array1<f32> = Array::range(start, end, step);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_linspace)]
pub fn linspace(start: f32, end: f32, n: usize) -> JsValue {
  let mat: Array<f32, _> = Array::linspace(start, end, n);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_logspace)]
pub fn logspace(base: f32, start: f32, end: f32, n: usize) -> JsValue {
  let mat: Array<f32, _> = Array::logspace(base, start, end, n);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_geomspace)]
pub fn geomspace(start: f32, end: f32, n: usize) -> JsValue {
  let mat: Array<f32, _> = Array::geomspace(start, end, n)
    .expect_throw("Geomspace cannot be generated with provided params.");
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_ones)]
pub fn ones(shape: JsValue) -> JsReturn {
  let shape = js_into!(shape, Vec<Ix>)?;
  let mat: ArrayD<f32> = ArrayD::ones(IxDyn(&shape));
  Ok(JsValue::from_serde(&mat).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_zeros)]
pub fn zeros(shape: JsValue) -> JsReturn {
  let shape = js_into!(shape, Vec<Ix>)?;
  let mat: ArrayD<f32> = ArrayD::zeros(IxDyn(&shape));
  Ok(JsValue::from_serde(&mat).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_from_el)]
pub fn from_el(shape: JsValue, element: f32) -> JsReturn {
  let shape = js_into!(shape, Vec<Ix>)?;
  let mat: ArrayD<f32> = ArrayD::from_elem(IxDyn(&shape), element);
  Ok(JsValue::from_serde(&mat).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_eye)]
pub fn eye(shape: usize) -> JsValue {
  let mat: Array<f32, _> = Array::eye(shape);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_from_diag)]
pub fn diag(ndarray: JsValue) -> JsReturn {
  let mat = js_into!(ndarray, Array<f32, Ix1>)?;
  let mat: Array<f32, _> = Array::from_diag(&mat);
  Ok(JsValue::from_serde(&mat).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_random)]
pub fn random(shape: JsValue) -> JsReturn {
  let shape = js_into!(shape, Vec<Ix>)?;
  // TODO(qu4k): look into distribution types
  let mat: ArrayD<f32> =
    ArrayD::from_shape_simple_fn(shape, || js_sys::Math::random() as f32);
  Ok(JsValue::from_serde(&mat).unwrap())
}
