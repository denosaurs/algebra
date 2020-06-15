// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;

use ndarray::{Ix, SliceInfo};

use crate::ndarray_helpers::JsReturn;
use crate::ndarray_slice::new_slice_info;

//
// array methods
//
#[wasm_bindgen(js_name = ndarray_f32_index)]
pub fn index(ndarray: JsValue, index: JsValue) -> JsReturn<f32> {
  let index = js_into!(index, Vec<Ix>)?;
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(mat[IxDyn(&index)])
}

#[wasm_bindgen(js_name = ndarray_f32_slice)]
pub fn slice(ndarray: JsValue, slice_info: Vec<JsValue>) -> JsReturn {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  let slice_info = new_slice_info(slice_info)
    .expect_throw("Cannot generate slice info from provided params.");
  let slice_info: SliceInfo<_, IxDyn> = match SliceInfo::new(slice_info) {
    Ok(mat) => mat,
    Err(e) => return Err(js_error!("{}", e.to_string())),
  };
  if slice_info.out_ndim() != mat.ndim() {
    return Err(js_error!("SliceInfo axes number ({}) does not match the number of array axes ({}).", slice_info.out_ndim(), mat.ndim()));
  }
  let mat = mat.slice(slice_info.as_ref());
  Ok(JsValue::from_serde(&mat).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_reshape)]
pub fn reshape(ndarray: JsValue, shape: JsValue) -> JsReturn {
  let shape = js_into!(shape, Vec<Ix>)?;
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  let mat = match mat.into_shape(shape) {
    Ok(mat) => mat,
    Err(e) => return Err(js_error!("{}", e.to_string())),
  };
  Ok(JsValue::from_serde(&mat).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_transpose)]
pub fn transpose(ndarray: JsValue) -> JsReturn {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  let mat = mat.t();
  Ok(JsValue::from_serde(&mat).unwrap())
}

#[wasm_bindgen(js_name = ndarray_f32_format)]
pub fn format(ndarray: JsValue) -> JsReturn<String> {
  let mat = js_into!(ndarray, ArrayD<f32>)?;
  Ok(format!("{}", mat))
}
