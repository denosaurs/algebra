// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

interface raw<T> {
  v: number;
  dim: Shape;
  data: T;
}

export class ndarray<T> {
  __raw: raw<T>;

  constructor(array: raw<T>) {
    this.__raw = array;
  }

  // info
  ndim(): number {
    throw new Error("not implemented");
  }

  // indexing and slicing
  idx(index: number | Idx[]) {
    throw new Error("not implemented");
  }
  slice(info: RangeOrIndex[]) {
    throw new Error("not implemented");
  }

  reshape(shape: Shape) {
    throw new Error("not implemented");
  }

  string() {
    throw new Error("not implemented");
  }
}

export type Shape = number[];
export type Idx = number;

export type RangeOrIndex = number | number[];
