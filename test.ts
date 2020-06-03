// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

import * as ab from "./mod.ts";

const mat = ab.fromFn(5, 5, (a, b) => {
  return (a) * (b);
});

const a = ab.get(mat, 3, 3);

console.log(mat, a);
