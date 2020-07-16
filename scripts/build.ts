// Copyright 2020-present the denosaurs team. All rights reserved. MIT license.

import { encode } from "https://deno.land/std@0.61.0/encoding/base64.ts";
import { join } from "https://deno.land/std@0.61.0/path/mod.ts";
import { compress } from "https://deno.land/x/lz4@v0.1.0/mod.ts";
import Terser from "https://jspm.dev/terser";

const encoder = new TextEncoder();

async function requires(...executables: string[]) {
  const where = Deno.build.os === "windows" ? "where" : "which";

  for (const executable of executables) {
    const process = Deno.run({
      cmd: [where, executable],
      stderr: "null",
      stdin: "null",
      stdout: "null",
    });

    if (!(await process.status()).success) {
      err(`Could not find required build tool ${executable}`);
    }
  }
}

async function run(msg: string, cmd: string[], wd: string) {
  log(msg);

  const process = Deno.run({
    cmd,
    cwd: wd,
  });

  if (!(await process.status()).success) {
    err(`${msg} failed`);
  }
}

function log(text: string): void {
  console.log(`[log] ${text}`);
}

function err(text: string): never {
  console.log(`[err] ${text}`);
  return Deno.exit(1);
}

await requires("rustup", "rustc", "cargo", "wasm-pack");

await build("linear", "algebra_linear", "linear_wasm.js");
await build("nd", "algebra_nd", "nd_wasm.js");

async function build(directory: string, crate: string, wasmFile: string) {
  if (!(await Deno.stat(join(directory, "Cargo.toml"))).isFile) {
    err(`the build script should be executed in the "${crate}" root`);
  }

  await run(
    "building using wasm-pack",
    ["wasm-pack", "build", "--target", "web", "--release"],
    directory,
  );

  const wasm = await Deno.readFile(join(directory, "pkg", `${crate}_bg.wasm`));
  const compressed = compress(wasm);
  log(
    `compressed wasm using lz4, size reduction: ${wasm.length -
      compressed.length} bytes`,
  );
  const encoded = encode(compressed);
  log(
    `encoded wasm using base64, size increase: ${encoded.length -
      compressed.length} bytes`,
  );

  log("inlining wasm in js");
  const source = `import * as lz4 from "https://deno.land/x/lz4@v0.1.0/mod.ts";
                  export const source = lz4.decompress(Uint8Array.from(atob("${encoded}"), c => c.charCodeAt(0)));`;

  const init = await Deno.readTextFile(join(directory, "pkg", `${crate}.js`));

  log("minifying js");
  const output = Terser.minify(`${source}\n${init}`, {
    mangle: { module: true },
    output: {
      preamble: "//deno-fmt-ignore-file",
    },
  });

  if (output.error) {
    err(`encountered error when minifying: ${output.error}`);
  }

  const reduction = new Blob([(`${source}\n${init}`)]).size -
    new Blob([output.code]).size;
  log(`minified js, size reduction: ${reduction} bytes`);

  log(`writing output to file ("${wasmFile}")`);
  await Deno.writeFile(wasmFile, encoder.encode(output.code));

  const outputFile = await Deno.stat(wasmFile);
  log(`output file ("${wasmFile}"), final size is: ${outputFile.size} bytes`);
}
