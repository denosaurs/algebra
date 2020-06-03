// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

import * as ab from "./mod.ts";

let b = ab.identity(3, 3);

ab.pretty(b);

console.log(ab.get(b, 2, 2));

b = ab.set(b, 2, 2, 22);

ab.pretty(b);

console.log(ab.get(b, 2, 2));
