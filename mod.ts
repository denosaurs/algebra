// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

import init, {
  source,
  dmatrix_f32_from_fn,
  dmatrix_f32_from_el,
  dmatrix_f32_zeros,
  dmatrix_f32_random,
  dmatrix_f32_identity,
  dmatrix_f32_set,
  dmatrix_f32_range,
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
  cols: number,
  fn: (r: number, c: number) => number,
): dMatrix<number> {
  return dmatrix_f32_from_fn(rows, cols, fn);
}

export function fromEl(
  rows: number,
  cols: number,
  element: number,
): dMatrix<number> {
  return dmatrix_f32_from_el(rows, cols, element);
}

export function identity(rows: number, cols: number): dMatrix<number> {
  return dmatrix_f32_identity(rows, cols);
}

export function zeros(rows: number, cols: number): dMatrix<number> {
  return dmatrix_f32_zeros(rows, cols);
}

export function random(rows: number, cols: number): dMatrix<number> {
  return dmatrix_f32_random(rows, cols);
}

export function range(
  rows: number,
  cols: number,
  start: number,
  end: number,
  step: number = 1,
): dMatrix<number> {
  if (rows * cols !== (end - start) / step) {
    throw Error("Range provided should be the same as row * cols");
  }
  return dmatrix_f32_range(rows, cols, start, end, step);
}

export function set<T>(
  dMatrix: dMatrix<T>,
  row: number,
  col: number,
  value: number,
): dMatrix<number> {
  return dmatrix_f32_set(dMatrix, row, col, value);
}

export function get<T>(dMatrix: dMatrix<T>, row: number, col: number): T {
  if (!(row >= 0 && row < dMatrix.nrows && col >= 0 && col < dMatrix.ncols)) {
    throw Error("Index provided should be the in the matrix");
  }
  return dmatrix_f32_get(dMatrix, row, col) as T;
}

// TODO(qu4k): debug only, remove or rework.
export function pretty<T>(dMatrix: dMatrix<T>) {
  let pretty = "";
  for (let row = 0; row < dMatrix.nrows; row++) {
    pretty += "|";
    for (let col = 0; col < dMatrix.ncols; col++) {
      pretty += ` ${dmatrix_f32_get(dMatrix, row, col)}`;
    }
    pretty += " |";
    if (row !== dMatrix.nrows - 1) pretty += "\n";
  }
  console.log(pretty);
}
