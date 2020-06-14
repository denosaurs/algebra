import * as ab from "../nd.ts";

let a = ab.range(1, 10, 1);
console.log(a.string());

a = a.reshape([3, 3]);
console.log(a.string());
console.log(a.T().string());
console.log(a.idx([1, 1]));

let b = ab.random([4, 4, 4]);
console.log(b.string());

const slice = Array(b.ndim()).fill(ab.Range(1, 3, 1));
let c = b.slice(slice);

console.log(c.string());

console.log(ab.array(1).string());
console.log(ab.array([1, 2, 3, 4]).string());
console.log(ab.array([[1, 2, 3, 4, 5], [5, 4, 3, 2, 1]]).string());
console.log(ab.array([[[1, 2], [2, 3]], [[3, 4], [5, 6]]]).string());
