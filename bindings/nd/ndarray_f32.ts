// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

import {
  ndarray_f32_range,
  ndarray_f32_linspace,
  ndarray_f32_logspace,
  ndarray_f32_geomspace,
  ndarray_f32_ones,
  ndarray_f32_zeros,
  ndarray_f32_from_el,
  ndarray_f32_eye,
  ndarray_f32_diag,
  ndarray_f32_random,
  ndarray_f32_ndim,
  ndarray_f32_index,
  ndarray_f32_slice,
  ndarray_f32_reshape,
  ndarray_f32_format,
} from "../../nd_wasm.js";

import { Idx, ndarray, Shape, RangeOrIndex } from "./ndarray.ts";

export class ndarrayf32 extends ndarray<Float32Array> {
  ndim(): number {
    return ndarray_f32_ndim(this.__raw);
  }

  idx(index: number | Idx[]) {
    if (typeof index === "number") index = [index];
    return ndarray_f32_index(this.__raw, index);
  }

  slice(info: RangeOrIndex[]) {
    return new ndarrayf32(ndarray_f32_slice(this.__raw, info));
  }

  reshape(shape: Shape) {
    return new ndarrayf32(ndarray_f32_reshape(this.__raw, shape));
  }

  string() {
    return ndarray_f32_format(this.__raw);
  }
}

export function range(
  start: number,
  end: number,
  step: number = 1,
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_range(start, end, step));
}

export function linspace(
  start: number,
  end: number,
  n: number,
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_linspace(start, end, n));
}

export function logspace(
  base: number,
  start: number,
  end: number,
  n: number,
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_logspace(base, start, end, n));
}

export function geomspace(
  start: number,
  end: number,
  n: number,
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_geomspace(start, end, n));
}

export function ones(shape: Shape): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_ones(shape));
}

export function zeroes(shape: Shape): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_zeros(shape));
}

export function from_el(shape: Shape, element: number): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_from_el(shape, element));
}

export function eye(shape: Shape): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_eye(shape));
}

export function diag(
  diag: ndarrayf32,
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_diag(diag));
}

export function random(
  shape: Shape,
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_random(shape));
}
