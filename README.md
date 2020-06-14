# algebra

---
> ⚠️ This project is work in progress. Expect breaking changes.
---

```typescript
import * as ab from "https://deno.land/x/algebra/nd.ts";

let a = ab.range(0, 2, 0.5);
console.log(a.string());

a = a.reshape([2, 2]);
console.log(a.string());
console.log(a.idx([1, 1]));

let b = ab.random([8, 8, 8]);
console.log(b.string());

const slice = Array(b.ndim()).fill(ab.Range(2, 6, 1));
let c = b.slice(slice);
console.log(c.string());
```
