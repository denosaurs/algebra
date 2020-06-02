// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

import init, {
  source,
  dmatrix_f32_from_fn,
  dmatrix_f32_zeros,
  dmatrix_f32_random,
  dmatrix_f32_get,
} from "./wasm.js";

await init(source);

interface dMatrix<T> {
  data: T[];
  nrows: number;
  ncols: number;
}

export function fromFn(
  rows: number,
  columns: number,
  fn: (r: number, c: number) => number,
): dMatrix<number> {
  return dmatrix_f32_from_fn(rows, columns, fn);
}

export function zeros(rows: number, columns: number): dMatrix<number> {
  return dmatrix_f32_zeros(rows, columns);
}

export function random(rows: number, columns: number): dMatrix<number> {
  return dmatrix_f32_random(rows, columns);
}

export function get<T>(dMatrix: dMatrix<T>, row: number, col: number): T {
  return dmatrix_f32_get(dMatrix, row, col) as T;
}
