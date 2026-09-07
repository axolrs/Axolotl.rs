// Owner: PascalElixir / axolrs (GitHub org)
// File: tooling/axolc-wasm/smoke.mjs - node smoke test: instantiate the wasm artifact and run the default playground program through the raw C ABI.

import { readFile } from 'node:fs/promises';

const DEFAULT_PROGRAM = `Player = struct
    name: String
    hp: Int
end

Player.attack = fn(self, damage: Int)
    self.hp = self.hp - damage
    return self.hp
end

fn main()
    var hero = Player { name = "axol", hp = 100 }
    hero:attack(12)
    print("hero hp = \${hero.hp}")
end`;

const wasmBytes = await readFile(new URL('./target/wasm32-unknown-unknown/release/axolc_wasm.wasm', import.meta.url));
const { instance } = await WebAssembly.instantiate(wasmBytes, {});
const api = instance.exports;

function readCString(memory, ptr) {
  const bytes = new Uint8Array(memory.buffer);
  let end = ptr;
  while (bytes[end] !== 0) end += 1;
  return new TextDecoder().decode(bytes.subarray(ptr, end));
}

function writeSource(memory, api, source) {
  const encoded = new TextEncoder().encode(source);
  const ptr = api.axolc_alloc(encoded.length);
  new Uint8Array(memory.buffer, ptr, encoded.length).set(encoded);
  return { ptr, len: encoded.length };
}

const version = readCString(api.memory, api.axolc_version());
if (version !== '0.1.0-moss') throw new Error(`unexpected version: ${version}`);
console.log(`version ok: axolc ${version} · wasm`);

const runPtr = writeSource(api.memory, api, DEFAULT_PROGRAM);
const runJson = readCString(api.memory, api.axolc_run(runPtr.ptr, runPtr.len));
const run = JSON.parse(runJson);
if (!run.output.includes('hero hp = 88')) throw new Error(`run output unexpected: ${JSON.stringify(run)}`);
if (run.diagnostics.length !== 0) throw new Error(`expected no diagnostics, got: ${JSON.stringify(run.diagnostics)}`);
console.log(`run ok: ${JSON.stringify(run.output)}`);

const broken = 'fn main()\n    let x =\nend';
const brokenPtr = writeSource(api.memory, api, broken);
const brokenJson = readCString(api.memory, api.axolc_run(brokenPtr.ptr, brokenPtr.len));
const brokenResult = JSON.parse(brokenJson);
const first = brokenResult.diagnostics[0];
if (!first || first.severity !== 'error' || !(first.line >= 1) || !(first.col >= 1)) {
  throw new Error(`expected positioned error diagnostic, got: ${JSON.stringify(brokenResult)}`);
}
console.log(`diagnostics ok: ${first.line}:${first.col} ${first.severity}: ${first.message}`);

const compilePtr = writeSource(api.memory, api, DEFAULT_PROGRAM);
const compileJson = readCString(api.memory, api.axolc_compile(compilePtr.ptr, compilePtr.len));
const compiled = JSON.parse(compileJson);
if (!compiled.rust.includes('fn main()')) throw new Error('compiled rust missing fn main()');
if (compiled.output !== '') throw new Error('compile output should be empty');
console.log(`compile ok: generated ${compiled.rust.split('\n').length} lines of rust`);
