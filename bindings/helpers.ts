// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

export function Range(start: number, end: number, step: number): number[] {
  return [start, end, step];
}

export function Int(start: number): number {
  return Math.round(start);
}
