// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;

use ndarray::Ix;
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;

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
pub fn ones(shape: JsValue) -> JsValue {
  let shape: Vec<Ix> = shape.into_serde().expect_throw("Shape is not valid.");
  let mat: ArrayD<f32> = ArrayD::ones(IxDyn(&shape));
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_zeros)]
pub fn zeros(shape: JsValue) -> JsValue {
  let shape: Vec<Ix> = shape.into_serde().expect_throw("Shape is not valid.");
  let mat: ArrayD<f32> = ArrayD::zeros(IxDyn(&shape));
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_from_el)]
pub fn from_el(shape: JsValue, element: f32) -> JsValue {
  let shape: Vec<Ix> = shape.into_serde().expect_throw("Shape is not valid.");
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
  let mat: Array<f32, _> =
    ndarray.into_serde().expect_throw("Diagonal is not valid.");
  let mat: Array<f32, _> = Array::from_diag(&mat);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_random)]
pub fn random(shape: JsValue) -> JsValue {
  let shape: Vec<Ix> = shape.into_serde().expect_throw("Shape is not valid.");
  // TODO(qu4k): look into distribution types
  let mat: ArrayD<f32> = ArrayD::random(IxDyn(&shape), Uniform::new(0., 1.));
  JsValue::from_serde(&mat).unwrap()
}
