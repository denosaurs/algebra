// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;

use crate::approx::AbsDiffEq;
use crate::ndarray_helpers::JsReturn;

#[wasm_bindgen(js_name = ndarray_f32_dot)]
pub fn dot(lhs: JsValue, rhs: JsValue) -> JsReturn {
  let lhs = js_into!(lhs, Array2<f32>)?;
  let rhs = js_into!(rhs, Array2<f32>)?;
  Ok(JsValue::from_serde(&lhs.dot(&rhs)).unwrap())
}

// sum
#[wasm_bindgen(js_name = ndarray_f32_sum_el)]
pub fn sum_el(lhs: JsValue, rhs: JsValue) -> JsReturn {
  let lhs = js_into!(lhs, ArrayD<f32>)?;
  let rhs = js_into!(rhs, f32)?;
  Ok(JsValue::from_serde(&(lhs + rhs)).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_sum_mat)]
pub fn sum_mat(lhs: JsValue, rhs: JsValue) -> JsReturn {
  let lhs = js_into!(lhs, ArrayD<f32>)?;
  let rhs = js_into!(rhs, ArrayD<f32>)?;
  Ok(JsValue::from_serde(&(lhs + rhs)).unwrap())
}

// sub
#[wasm_bindgen(js_name = ndarray_f32_sub_el)]
pub fn sub_el(lhs: JsValue, rhs: JsValue) -> JsReturn {
  let lhs = js_into!(lhs, ArrayD<f32>)?;
  let rhs = js_into!(rhs, f32)?;
  Ok(JsValue::from_serde(&(lhs - rhs)).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_sub_mat)]
pub fn sub_mat(lhs: JsValue, rhs: JsValue) -> JsReturn {
  let lhs = js_into!(lhs, ArrayD<f32>)?;
  let rhs = js_into!(rhs, ArrayD<f32>)?;
  Ok(JsValue::from_serde(&(lhs - rhs)).unwrap())
}

// mul
#[wasm_bindgen(js_name = ndarray_f32_mul_el)]
pub fn mul_el(lhs: JsValue, rhs: JsValue) -> JsReturn {
  let lhs = js_into!(lhs, ArrayD<f32>)?;
  let rhs = js_into!(rhs, f32)?;
  Ok(JsValue::from_serde(&(lhs * rhs)).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_mul_mat)]
pub fn mul_mat(lhs: JsValue, rhs: JsValue) -> JsReturn {
  let lhs = js_into!(lhs, ArrayD<f32>)?;
  let rhs = js_into!(rhs, ArrayD<f32>)?;
  Ok(JsValue::from_serde(&(lhs * rhs)).unwrap())
}

// div
#[wasm_bindgen(js_name = ndarray_f32_div_el)]
pub fn div_el(lhs: JsValue, rhs: JsValue) -> JsReturn {
  let lhs = js_into!(lhs, ArrayD<f32>)?;
  let rhs = js_into!(rhs, f32)?;
  Ok(JsValue::from_serde(&(lhs / rhs)).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_div_mat)]
pub fn div_mat(lhs: JsValue, rhs: JsValue) -> JsReturn {
  let lhs = js_into!(lhs, ArrayD<f32>)?;
  let rhs = js_into!(rhs, ArrayD<f32>)?;
  Ok(JsValue::from_serde(&(lhs / rhs)).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_mapv)]
pub fn mapv(ndarray: JsValue, fun: &js_sys::Function) -> JsReturn {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  let mat: ArrayD<f32> = mat.mapv(|a| {
    let res = fun
      .call1(&ndarray, &JsValue::from(a as f32))
      .unwrap();
    res.as_f64().unwrap() as f32
  });
  Ok(JsValue::from_serde(&mat).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_sum)]
pub fn sum(ndarray: JsValue) -> JsReturn {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(JsValue::from_serde(&mat.sum()).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_sum_axis)]
pub fn sum_axis(ndarray: JsValue, axis: usize) -> JsReturn {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(JsValue::from_serde(&mat.sum_axis(Axis(axis))).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_mean)]
pub fn mean(ndarray: JsValue) -> JsReturn {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(JsValue::from_serde(&mat.mean()).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_mean_axis)]
pub fn mean_axis(ndarray: JsValue, axis: usize) -> JsReturn {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(JsValue::from_serde(&mat.mean_axis(Axis(axis))).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_allclose)]
pub fn allclose(lhs: JsValue, rhs: JsValue, epsilon: f32) -> JsReturn {
  let lhs = js_into!(lhs, ArrayD<f32>)?;
  let rhs = js_into!(rhs, ArrayD<f32>)?;
  Ok(JsValue::from_serde(&lhs.abs_diff_eq(&rhs, epsilon)).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_diag)]
pub fn diag(ndarray: JsValue) -> JsReturn {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(JsValue::from_serde(&mat.diag()).unwrap())
}
