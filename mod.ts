import init, {
  source,
  vector as wasm_vector,
} from "./wasm.js";

await init(source);

export function vector() {
  return wasm_vector();
}
