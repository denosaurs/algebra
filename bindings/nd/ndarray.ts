// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

interface raw<T> {
  v: number;
  dim: Shape;
  data: number[];
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

  len(): number {
    throw new Error("not implemented");
  }

  shape(): Shape {
    throw new Error("not implemented");
  }

  lenOf(axis: Axis): number {
    throw new Error("not implemented");
  }

  strides(): Stride[] {
    throw new Error("not implemented");
  }

  empty(): boolean {
    throw new Error("not implemented");
  }

  // indexing & slicing
  idx(index: number | Idx[]): number {
    throw new Error("not implemented");
  }

  slice(info: RangeOrIndex[]): ndarray<T> {
    throw new Error("not implemented");
  }

  reshape(shape: Shape): ndarray<T> {
    throw new Error("not implemented");
  }

  transpose(): ndarray<T> {
    throw new Error("not implemented");
  }

  T(): ndarray<T> {
    return this.transpose();
  }

  string(): string {
    throw new Error("not implemented");
  }

  // operations
  dot(rhs: ndarray<T>): ndarray<T> {
    throw new Error("not implemented");
  }
}

export type Shape = number[];
export type Stride = number;
export type Idx = number;
export type Axis = number;

export type RangeOrIndex = number | number[];
