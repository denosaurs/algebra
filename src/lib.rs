// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

// https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html

#[macro_use]
extern crate serde_derive;

extern crate nalgebra as na;

use na::Vector3;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn vector() -> JsValue {
    let vec = Vector3::new(0.0, 0.0, 1.0);
    JsValue::from_serde(&vec).unwrap()
}
