// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

interface raw<T> {
  v: number;
  dim: Shape;
  data: number[];
}

export abstract class ndarray<T> {
  __raw: raw<T>;

  constructor(array: raw<T>) {
    this.__raw = array;
  }

  // info
  abstract ndim(): number;
  abstract len(): number;
  abstract shape(): Shape;
  abstract lenOf(axis: Axis): number;
  abstract strides(): Stride[];
  abstract empty(): boolean;

  // indexing & slicing
  abstract idx(index: number | Idx[]): number;
  abstract slice(info: RangeOrIndex[]): ndarray<T>;
  abstract reshape(shape: Shape): ndarray<T>;
  abstract transpose(): ndarray<T>;
  T(): ndarray<T> {
    return this.transpose();
  }
  abstract string(): string;

  // operations
  abstract dot(rhs: ndarray<T>): ndarray<T>;
  abstract sum(rhs: ndarray<T> | number): ndarray<T>;
  abstract sub(rhs: ndarray<T> | number): ndarray<T>;
  abstract mul(rhs: ndarray<T> | number): ndarray<T>;
  abstract div(rhs: ndarray<T> | number): ndarray<T>;
  abstract map(fn: (val: number) => number): ndarray<T>;
  abstract sumAll(): number;
  abstract sumAxis(axis: number): number;
  abstract mean(): number;
  abstract meanAxis(axis: number): number;
  abstract allClose(rhs: ndarray<T>, epsilon: number): boolean;
  abstract diag(): ndarray<T>;
}

export type Shape = number[];
export type Stride = number;
export type Idx = number;
export type Axis = number;

export type RangeOrIndex = number | number[];
