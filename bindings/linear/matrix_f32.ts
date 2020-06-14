// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

import {
  dmatrix_f32_from_fn,
  dmatrix_f32_from_el,

  dmatrix_f32_identity,
  dmatrix_f32_zeros,
  dmatrix_f32_random,
  dmatrix_f32_range,

  dmatrix_f32_get,
  dmatrix_f32_set,
  dmatrix_f32_min,
  dmatrix_f32_max,
} from "../../linear_wasm.js";
import { dMatrix } from "./dmatrix.ts";

export function fromFn(
  rows: number,
  cols: number,
  fn: (r: number, c: number) => number,
): dMatrix<Float32Array> {
  return dmatrix_f32_from_fn(rows, cols, fn);
}

export function fromEl(
  rows: number,
  cols: number,
  element: number,
): dMatrix<Float32Array> {
  return dmatrix_f32_from_el(rows, cols, element);
}

export function identity(rows: number, cols: number): dMatrix<Float32Array> {
  return dmatrix_f32_identity(rows, cols);
}

export function zeros(rows: number, cols: number): dMatrix<Float32Array> {
  return dmatrix_f32_zeros(rows, cols);
}

export function random(rows: number, cols: number): dMatrix<Float32Array> {
  return dmatrix_f32_random(rows, cols);
}

export function range(
  rows: number,
  cols: number,
  start: number,
  end: number,
  step: number = 1,
): dMatrix<Float32Array> {
  if (rows * cols !== (end - start) / step) {
    throw Error("Range provided should be the same as row * cols");
  }
  return dmatrix_f32_range(rows, cols, start, end, step);
}

export function get<T>(dMatrix: dMatrix<T>, row: number, col: number): number {
  if (!(row >= 0 && row < dMatrix.nrows && col >= 0 && col < dMatrix.ncols)) {
    throw Error("Index provided should be the in the matrix");
  }
  return dmatrix_f32_get(dMatrix, row, col) as number;
}

export function set<T>(
  dMatrix: dMatrix<T>,
  row: number,
  col: number,
  value: number,
): dMatrix<Float32Array> {
  return dmatrix_f32_set(dMatrix, row, col, value);
}

export function max<T>(dMatrix: dMatrix<T>): number {
  return dmatrix_f32_max(dMatrix);
}

export function min<T>(dMatrix: dMatrix<T>): number {
  return dmatrix_f32_min(dMatrix);
}
