// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;

use crate::ndarray_slice::new_slice_info;
use ndarray::{Ix, SliceInfo};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;

//
// js hooks, // TODO(qu4k): implement betterc
//
#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);

  #[wasm_bindgen(js_namespace = console, js_name = log)]
  fn logVal(s: &JsValue);
}

//
// array creation
//
#[wasm_bindgen(js_name = ndarray_f32_range)]
pub fn range(start: f32, end: f32, step: f32) -> JsValue {
  let mat: Array<f32, _> = Array::range(start, end, step);
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
  let mat: Array<f32, _> = Array::geomspace(start, end, n).unwrap();
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_ones)]
pub fn ones(shape: JsValue) -> JsValue {
  let shape: Vec<Ix> = shape.into_serde().unwrap();
  let mat: ArrayD<f32> = ArrayD::ones(IxDyn(&shape));
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_zeros)]
pub fn zeros(shape: JsValue) -> JsValue {
  let shape: Vec<Ix> = shape.into_serde().unwrap();
  let mat: ArrayD<f32> = ArrayD::zeros(IxDyn(&shape));
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_from_el)]
pub fn from_el(shape: JsValue, element: f32) -> JsValue {
  let shape: Vec<Ix> = shape.into_serde().unwrap();
  let mat: ArrayD<f32> = ArrayD::from_elem(IxDyn(&shape), element);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_eye)]
pub fn eye(shape: usize) -> JsValue {
  let mat: Array<f32, _> = Array::eye(shape);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_diag)]
pub fn diag(ndarray: JsValue) -> JsValue {
  let mat: Array<f32, _> = ndarray.into_serde().unwrap();
  let mat: Array<f32, _> = Array::from_diag(&mat);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_random)]
pub fn random(shape: JsValue) -> JsValue {
  let shape: Vec<Ix> = shape.into_serde().unwrap();
  let mat: ArrayD<f32> = ArrayD::random(IxDyn(&shape), Uniform::new(0., 1.));
  JsValue::from_serde(&mat).unwrap()
}

//
// array info
//
#[wasm_bindgen(js_name = ndarray_f32_ndim)]
pub fn ndim(ndarray: JsValue) -> usize {
  let mat: ArrayD<f32> = ndarray.into_serde().unwrap();
  mat.ndim()
}

//
// array methods
//
#[wasm_bindgen(js_name = ndarray_f32_index)]
pub fn index(ndarray: JsValue, index: JsValue) -> f32 {
  let index: Vec<Ix> = index.into_serde().unwrap();
  let mat: ArrayD<f32> = ndarray.into_serde().unwrap();
  mat[IxDyn(&index)]
}

#[wasm_bindgen(js_name = ndarray_f32_slice)]
pub fn slice(ndarray: JsValue, slice_info: Vec<JsValue>) -> JsValue {
  let mat: ArrayD<f32> = ndarray.into_serde().unwrap();
  let slice_info = new_slice_info(slice_info).unwrap();
  let slice_info: SliceInfo<_, IxDyn> = SliceInfo::new(slice_info).unwrap();

  JsValue::from_serde(&mat.slice(slice_info.as_ref())).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_reshape)]
pub fn reshape(ndarray: JsValue, shape: JsValue) -> JsValue {
  let shape: Vec<Ix> = shape.into_serde().unwrap();
  let mat: ArrayD<f32> = ndarray.into_serde().unwrap();
  let mat = mat.into_shape(IxDyn(&shape)).unwrap();
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_format)]
pub fn format(ndarray: JsValue) -> String {
  let mat: ArrayD<f32> = ndarray.into_serde().unwrap();
  format!("{}", mat)
}
