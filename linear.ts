import init, {
  source,
} from "./linear_wasm.js";

await init(source);

console.warn(
  "[!] algebra `linear` bindings are experimental at this stage and will probably be removed.",
);

export { dMatrix } from "./bindings/linear/dmatrix.ts";
export * as f32 from "./bindings/linear/matrix_f32.ts";

export * from "./bindings/linear/matrix_f32.ts";
export * from "./bindings/helpers.ts";
