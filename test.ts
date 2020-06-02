// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

import * as nj from "./mod.ts";

const mat = nj.fromFn(5, 5, (a, b) => {
  return (a) * (b);
});

const a = nj.get(mat, 3, 3);

console.log(mat, a);
