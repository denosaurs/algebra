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

#[wasm_bindgen(js_name = dmatrix_f32_zeros)]
pub fn zeros(rows: usize, cols: usize) -> JsValue {
    let mat = DMatrix::<f32>::zeros(rows, cols);
    JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = dmatrix_f32_random)]
pub fn random(rows: usize, cols: usize) -> JsValue {
    let mat = DMatrix::<f32>::from_fn(rows, cols, |_, _| rand::random());
    JsValue::from_serde(&mat).unwrap()
}

#[wasm_bindgen(js_name = dmatrix_f32_get)]
pub fn get(matrix: &JsValue, rows: usize, cols: usize) -> f32 {
    let mat: DMatrix<f32> = matrix.into_serde().unwrap();
    mat.index((rows, cols)).clone()
}
