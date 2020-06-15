import * as ab from "../nd.ts";

let A = ab.random([2, 2]);
console.log(A.dot(A.T()).string());
