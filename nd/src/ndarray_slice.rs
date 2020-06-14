// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

use ndarray::SliceOrIndex;
use wasm_bindgen::JsValue;

use crate::AlgebraError;

fn slice_info_slice(slice: &Vec<isize>) -> Result<SliceOrIndex, AlgebraError> {
  if slice.len() < 1 {
    return Err(AlgebraError);
  }
  Ok(SliceOrIndex::Slice {
    start: slice.get(0).cloned().unwrap(),
    end: slice.get(1).cloned(),
    step: slice.get(2).cloned().unwrap_or(1),
  })
}

fn slice_info_index(n: &JsValue) -> Result<SliceOrIndex, AlgebraError> {
  n.as_f64()
    .ok_or(AlgebraError)
    .and_then(|n| Ok(SliceOrIndex::Index(n as isize)))
}

pub fn new_slice_info(
  info: Vec<JsValue>,
) -> Result<Vec<SliceOrIndex>, AlgebraError> {
  let mut v = vec![];
  for value in &info {
    if value.is_object() {
      let elements: Vec<isize> = value.into_serde().unwrap();
      match slice_info_slice(&elements) {
        Ok(s) => v.push(s),
        Err(e) => return Err(e),
      }
    } else {
      match slice_info_index(value) {
        Ok(s) => v.push(s),
        Err(e) => return Err(e),
      }
    }
  }
  Ok(v)
}
