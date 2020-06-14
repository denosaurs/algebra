import init, {
  source,
} from "./nd_wasm.js";

await init(source);

export * as f32 from "./bindings/nd/ndarray_f32.ts";

export * from "./bindings/nd/ndarray_f32.ts";
export * from "./bindings/helpers.ts";
