// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

import {
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
  ndarray_f32_from_diag,

  ndarray_f32_dot,
} from "../../../nd_wasm.js";

import { ndarrayf32 } from "./interface.ts";
import { Shape } from "../ndarray.ts";

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

export function arange(
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

export function full(shape: Shape, element: number): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_from_el(shape, element));
}

export function eye(shape: Shape): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_eye(shape));
}

export function diag(
  diag: ndarrayf32,
): ndarrayf32 {
  return new ndarrayf32(ndarray_f32_from_diag(diag));
}

// Random is a bit special as it's implemented in
// pure js so performance does not suck.
// TODO(qu4k): look into performance
export function random(
  shape: Shape,
): ndarrayf32 {
  return new ndarrayf32({
    v: 1,
    dim: shape,
    data: Array.from({
      length: shape.reduce((a, b) => a * b),
    }, () => {
      return Math.random();
    }),
  });
}

export function dot(lhs: ndarrayf32, rhs: ndarrayf32) {
  return new ndarrayf32(ndarray_f32_dot(lhs.__raw, rhs.__raw));
}
