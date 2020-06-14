// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use na::DMatrix;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen(js_name = dmatrix_f32_from_fn)]
pub fn from_fn(rows: usize, cols: usize, f: &js_sys::Function) -> JsValue {
  let this = JsValue::NULL;
  let mat = DMatrix::<f32>::from_fn(rows, cols, |a, b| {
    let res = f
      .call2(&this, &JsValue::from(a as u32), &JsValue::from(b as u32))
      .unwrap();
    res.as_f64().unwrap() as f32
  });
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = dmatrix_f32_from_el)]
pub fn from_el(rows: usize, cols: usize, element: f32) -> JsValue {
  let mat = DMatrix::<f32>::from_element(rows, cols, element);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = dmatrix_f32_zeros)]
pub fn zeros(rows: usize, cols: usize) -> JsValue {
  let mat = DMatrix::<f32>::zeros(rows, cols);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = dmatrix_f32_ones)]
pub fn ones(rows: usize, cols: usize) -> JsValue {
  let mat = DMatrix::<f32>::from_element(rows, cols, 1.0);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = dmatrix_f32_identity)]
pub fn identity(rows: usize, cols: usize) -> JsValue {
  let mat = DMatrix::<f32>::identity(rows, cols);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = dmatrix_f32_random)]
pub fn random(rows: usize, cols: usize) -> JsValue {
  let mat = DMatrix::<f32>::from_fn(rows, cols, |_, _| rand::random());
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = dmatrix_f32_range)]
pub fn range(
  rows: usize,
  cols: usize,
  start: usize,
  end: usize,
  step: usize,
) -> JsValue {
  fn conv(x: usize) -> f32 {
    x as f32
  }
  let mat = DMatrix::<f32>::from_vec(
    rows,
    cols,
    (start..end).step_by(step).map(conv).collect(),
  );
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = dmatrix_f32_get)]
pub fn get(matrix: &JsValue, rows: usize, cols: usize) -> f32 {
  let mat: DMatrix<f32> = matrix.into_serde().unwrap();
  mat[(rows, cols)]
}

#[wasm_bindgen(js_name = dmatrix_f32_set)]
pub fn set(matrix: &JsValue, rows: usize, cols: usize, value: f32) -> JsValue {
  let mut mat: DMatrix<f32> = matrix.into_serde().unwrap();
  mat[(rows, cols)] = value;
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = dmatrix_f32_max)]
pub fn max(matrix: &JsValue) -> f32 {
  let mat: DMatrix<f32> = matrix.into_serde().unwrap();
  mat.max()
}

#[wasm_bindgen(js_name = dmatrix_f32_min)]
pub fn min(matrix: &JsValue) -> f32 {
  let mat: DMatrix<f32> = matrix.into_serde().unwrap();
  mat.min()
}
