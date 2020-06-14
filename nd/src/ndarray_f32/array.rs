// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;

use ndarray::arr1;

//
// array creation
//
#[wasm_bindgen(js_name = ndarray_f32_arr0)]
pub fn array(value: f32) -> JsValue {
  let mat: Array<f32, _> = arr0(value);
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_arr1)]
pub fn array1(array: Vec<f32>) -> JsValue {
  let mat: Array<f32, _> = arr1(array.as_slice());
  JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = ndarray_f32_arr2)]
pub fn array2(array: JsValue) -> Result<JsValue, JsValue> {
  let array: Vec<Vec<f32>> = array
    .into_serde()
    .expect_throw("Array should be a number[][]");
  let mut flattened = vec![];

  let x = array.len();
  let mut y = None;

  for axis in &array {
    if y.is_none() {
      y = Some(axis.len());
    } else if y.unwrap() != axis.len() {
      return Err(js_error!("{}", "Error"));
    }
    flattened.extend(axis);
  }
  if y.is_none() {
    return Err(js_error!("{}", "Error"));
  }
  let y = y.unwrap();

  let mat: ArrayD<f32> =
    ArrayD::from_shape_vec(IxDyn(&[x, y]), flattened).unwrap();
  Ok(JsValue::from_serde(&mat).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_arr3)]
pub fn array3(array: JsValue) -> Result<JsValue, JsValue> {
  let array: Vec<Vec<Vec<f32>>> = array
    .into_serde()
    .expect_throw("Array should be a number[][][]");
  let mut flattened = vec![];

  let x = array.len();
  let mut y = None;
  let mut z = None;

  for y_axis in &array {
    if y.is_none() {
      y = Some(y_axis.len());
    } else if y.unwrap() != y_axis.len() {
      return Err(js_error!("{}", "Error"));
    }
    for z_axis in y_axis {
      if z.is_none() {
        z = Some(z_axis.len());
      } else if z.unwrap() != z_axis.len() {
        return Err(js_error!("{}", "Error"));
      }
      flattened.extend(z_axis);
    }
  }
  if y.is_none() || z.is_none() {
    return Err(js_error!("{}", "Error"));
  }
  let y = y.unwrap();
  let z = z.unwrap();

  let mat: ArrayD<f32> =
    ArrayD::from_shape_vec(IxDyn(&[x, y, z]), flattened).unwrap();
  Ok(JsValue::from_serde(&mat).unwrap())
}
