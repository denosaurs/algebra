// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

/// Slicing info parsing & generation
use ndarray::SliceOrIndex;
use wasm_bindgen::{JsValue, UnwrapThrowExt};

use crate::AlgebraError;

/// Get slice from js array
/// js array should be in the form of `[start: number, end: number, step: number]`
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

/// Get index from js value
fn slice_info_index(n: &JsValue) -> Result<SliceOrIndex, AlgebraError> {
  n.as_f64()
    .ok_or(AlgebraError)
    .and_then(|n| Ok(SliceOrIndex::Index(n as isize)))
}

/// Parse slice_info
pub fn new_slice_info(
  info: Vec<JsValue>,
) -> Result<Vec<SliceOrIndex>, AlgebraError> {
  let mut v = vec![];
  for value in &info {
    if value.is_object() {
      let elements: Vec<isize> = value
        .into_serde()
        .expect_throw("Range should be a number[3?]");
      let slice =
        slice_info_slice(&elements).expect_throw("Unable to parse range");
      v.push(slice);
    } else {
      let index =
        slice_info_index(value).expect_throw("Index not correctly formatted");
      v.push(index);
    }
  }
  Ok(v)
}
