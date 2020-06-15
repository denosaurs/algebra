// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

// serializing data with:
// https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html

extern crate ndarray;
extern crate approx;
extern crate serde_derive;

extern crate js_sys;
extern crate wasm_bindgen;

#[derive(Debug)]
pub struct AlgebraError;

#[macro_use]
mod ndarray_helpers;

pub mod ndarray_f32;
pub mod ndarray_slice;
