// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

import {
  ndarray_f32_ndim,
  ndarray_f32_len,
  ndarray_f32_shape,
  ndarray_f32_len_of,
  ndarray_f32_strides,
  ndarray_f32_is_empty,

  ndarray_f32_index,
  ndarray_f32_slice,
  ndarray_f32_reshape,
  ndarray_f32_transpose,
  ndarray_f32_format,

  ndarray_f32_dot,

  ndarray_f32_arr0,
  ndarray_f32_arr1,
  ndarray_f32_arr2,
  ndarray_f32_arr3,

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
} from "../../nd_wasm.js";

import { ndarray, Idx, RangeOrIndex, Shape, Axis, Stride } from "./ndarray.ts";

export class ndarrayf32 extends ndarray<Float32Array> {
  ndim(): number {
    return ndarray_f32_ndim(this.__raw);
  }

  len(): number {
    return ndarray_f32_len(this.__raw);
  }

  shape(): Shape {
    return ndarray_f32_shape(this.__raw);
  }

  lenOf(axis: Axis): number {
    return ndarray_f32_len_of(this.__raw, axis);
  }

  strides(): Stride[] {
    return ndarray_f32_strides(this.__raw);
  }

  empty(): boolean {
    return ndarray_f32_is_empty(this.__raw);
  }

  idx(index: number | Idx[]): number {
    if (typeof index === "number") index = [index];
    return ndarray_f32_index(this.__raw, index);
  }

  slice(info: RangeOrIndex[]): ndarrayf32 {
    return new ndarrayf32(ndarray_f32_slice(this.__raw, info));
  }

  reshape(shape: Shape): ndarrayf32 {
    return new ndarrayf32(ndarray_f32_reshape(this.__raw, shape));
  }

  transpose(): ndarrayf32 {
    return new ndarrayf32(ndarray_f32_transpose(this.__raw));
  }

  string() {
    return ndarray_f32_format(this.__raw);
  }

  dot(rhs: ndarrayf32): ndarrayf32 {
    return new ndarrayf32(ndarray_f32_dot(this.__raw, rhs.__raw));
  }
}

export function arr0(
  element: number,
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_arr0(element));
}

export function arr1(
  array: number[],
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_arr1(array));
}

export function arr2(
  array: number[][],
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_arr2(array));
}

export function arr3(
  array: number[][][],
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_arr3(array));
}

type InitializeArray = number | number[] | number[][] | number[][][];

export function array(array: InitializeArray): ndarrayf32 {
  if (Array.isArray(array)) {
    if (Array.isArray(array[0])) {
      if (Array.isArray(array[0][0])) {
        return arr3(array as number[][][]);
      }
      return arr2(array as number[][]);
    }
    return arr1(array as number[]);
  }
  return arr0(array);
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

export function dot(lhs: ndarrayf32, rhs: ndarrayf32) {
  return new ndarrayf32(ndarray_f32_dot(lhs.__raw, rhs.__raw));
}
