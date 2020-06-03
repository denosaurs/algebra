# algebruh

---
> ⚠️ This project is work in progress. Expect breaking changes.
---

```typescript
import * as ab from "./mod.ts";

const mat = ab.fromFn(5, 5, (a, b) => {
  return (a) * (b);
});

const a = ab.get(mat, 3, 3);

console.log(mat, a);
```
