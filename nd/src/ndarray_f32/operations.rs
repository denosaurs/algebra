// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;

use crate::ndarray_helpers::JsReturn;

#[wasm_bindgen(js_name = ndarray_f32_dot)]
pub fn dot(lhs: JsValue, rhs: JsValue) -> JsReturn {
  let lhs = js_into!(lhs, Array2<f32>)?;
  let rhs = js_into!(rhs, Array2<f32>)?;
  Ok(JsValue::from_serde(&lhs.dot(&rhs)).unwrap())
}

// #[wasm_bindgen(js_name = ndarray_f32_sum_el)]
// pub fn sum_el(lhs: JsValue, rhs: JsValue) -> JsReturn {
//   let lhs: ArrayD<f32> = match lhs.into_serde() {
//     Ok(lhs) => lhs,
//     Err(e) => return Err(js_error!("{}", &e.to_string())),
//   };
//   let rhs: Array2<f32> = match rhs.into_serde() {
//     Ok(lhs) => lhs,
//     Err(e) => return Err(js_error!("{}", &e.to_string())),
//   };
//   lhs + rhs;
//   Ok(JsValue::from_serde(&lhs.dot(&rhs)).unwrap())
// }
//
// #[wasm_bindgen(js_name = ndarray_f32_sub_el)]
// pub fn sub_el(lhs: JsValue, rhs: JsValue) -> JsReturn {
//   let lhs: Array2<f32> = match lhs.into_serde() {
//     Ok(lhs) => lhs,
//     Err(e) => return Err(js_error!("POLO {}", &e.to_string())),
//   };
//   let rhs: Array2<f32> = match rhs.into_serde() {
//     Ok(lhs) => lhs,
//     Err(e) => return Err(js_error!("{}", &e.to_string())),
//   };
//   Ok(JsValue::from_serde(&lhs.dot(&rhs)).unwrap())
// }
//
// #[wasm_bindgen(js_name = ndarray_f32_mul_el)]
// pub fn mul_el(lhs: JsValue, rhs: JsValue) -> Result<JsValue, JsValue> {
//   let lhs: Array2<f32> = match lhs.into_serde() {
//     Ok(lhs) => lhs,
//     Err(e) => return Err(js_error!("POLO {}", &e.to_string())),
//   };
//   let rhs: Array2<f32> = match rhs.into_serde() {
//     Ok(lhs) => lhs,
//     Err(e) => return Err(js_error!("{}", &e.to_string())),
//   };
//   Ok(JsValue::from_serde(&lhs.dot(&rhs)).unwrap())
// }
//
// #[wasm_bindgen(js_name = ndarray_f32_dev_el)]
// pub fn dev_el(lhs: JsValue, rhs: JsValue) -> Result<JsValue, JsValue> {
//   let lhs: Array2<f32> = match lhs.into_serde() {
//     Ok(lhs) => lhs,
//     Err(e) => return Err(js_error!("POLO {}", &e.to_string())),
//   };
//   let rhs: Array2<f32> = match rhs.into_serde() {
//     Ok(lhs) => lhs,
//     Err(e) => return Err(js_error!("{}", &e.to_string())),
//   };
//   Ok(JsValue::from_serde(&lhs.dot(&rhs)).unwrap())
// }
