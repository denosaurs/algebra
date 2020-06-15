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
  ndarray_f32_sum_el,
  ndarray_f32_sum_mat,
  ndarray_f32_sub_el,
  ndarray_f32_sub_mat,
  ndarray_f32_mul_el,
  ndarray_f32_mul_mat,
  ndarray_f32_div_el,
  ndarray_f32_div_mat,
  ndarray_f32_mapv,
  ndarray_f32_sum,
  ndarray_f32_sum_axis,
  ndarray_f32_mean,
  ndarray_f32_mean_axis,
  ndarray_f32_allclose,
  ndarray_f32_diag,
} from "../../../nd_wasm.js";

import { ndarray, Idx, RangeOrIndex, Shape, Axis, Stride } from "../ndarray.ts";

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

  sum(rhs: ndarrayf32 | number): ndarrayf32 {
    if (typeof rhs === "number") {
      return new ndarrayf32(ndarray_f32_sum_el(this.__raw, rhs));
    }
    return new ndarrayf32(ndarray_f32_sum_mat(this.__raw, rhs.__raw));
  }

  sub(rhs: ndarrayf32 | number): ndarrayf32 {
    if (typeof rhs === "number") {
      return new ndarrayf32(ndarray_f32_sub_el(this.__raw, rhs));
    }
    return new ndarrayf32(ndarray_f32_sub_mat(this.__raw, rhs.__raw));
  }

  mul(rhs: ndarrayf32 | number): ndarrayf32 {
    if (typeof rhs === "number") {
      return new ndarrayf32(ndarray_f32_mul_el(this.__raw, rhs));
    }
    return new ndarrayf32(ndarray_f32_mul_mat(this.__raw, rhs.__raw));
  }

  div(rhs: ndarrayf32 | number): ndarrayf32 {
    if (typeof rhs === "number") {
      return new ndarrayf32(ndarray_f32_div_el(this.__raw, rhs));
    }
    return new ndarrayf32(ndarray_f32_div_mat(this.__raw, rhs.__raw));
  }

  map(fn: (val: number) => number): ndarrayf32 {
    return new ndarrayf32(ndarray_f32_mapv(this.__raw, fn));
  }

  sumAll(): number {
    return ndarray_f32_sum(this.__raw);
  }

  sumAxis(axis: number): number {
    return ndarray_f32_sum_axis(this.__raw, axis);
  }

  mean(): number {
    return ndarray_f32_mean(this.__raw);
  }

  meanAxis(axis: number): number {
    return ndarray_f32_mean_axis(this.__raw, axis);
  }

  allClose(rhs: ndarrayf32, epsilon: number): boolean {
    return ndarray_f32_allclose(this.__raw, rhs.__raw, epsilon);
  }

  diag(): ndarrayf32 {
    return new ndarrayf32(ndarray_f32_diag(this.__raw));
  }
}
