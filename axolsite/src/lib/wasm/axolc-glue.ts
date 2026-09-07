/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Typed loader for the axolc wasm module - instantiates axolc.wasm and wraps its flat C-ABI exports as safe string-in/string-out functions.
*/

export type DiagnosticSeverity = 'error' | 'warning' | 'info' | 'hint';

export interface PlaygroundDiagnostic {
	severity: DiagnosticSeverity;
	code: string | null;
	message: string;
	notes: string[];
	line: number;
	col: number;
}

export interface RunResult {
	output: string;
	diagnostics: PlaygroundDiagnostic[];
}

export interface CompileResult {
	output: string;
	rust: string;
	diagnostics: PlaygroundDiagnostic[];
}

export interface Axolc {
	version(): string;
	runProgram(src: string): RunResult;
	compileProgram(src: string): CompileResult;
}

interface WasmExports {
	memory: WebAssembly.Memory;
	axolc_alloc(len: number): number;
	axolc_version(): number;
	axolc_run(src: number, srcLen: number): number;
	axolc_compile(src: number, srcLen: number): number;
}

const WASM_ASSET_PATH = '/axolc.wasm';
const SEVERITIES: readonly DiagnosticSeverity[] = ['error', 'warning', 'info', 'hint'];

let loadPromise: Promise<Axolc> | null = null;

export function loadAxolc(): Promise<Axolc> {
	if (!loadPromise) {
		loadPromise = instantiate().catch((cause: unknown) => {
			loadPromise = null;
			throw cause;
		});
	}
	return loadPromise;
}

async function instantiate(): Promise<Axolc> {
	let bytes: ArrayBuffer;
	try {
		const response = await fetch(WASM_ASSET_PATH);
		if (!response.ok) {
			throw new Error(`HTTP ${response.status} ${response.statusText}`);
		}
		bytes = await response.arrayBuffer();
	} catch (cause) {
		throw new Error(
			`Could not load the Axolotl compiler (${WASM_ASSET_PATH}). Reload the page and try again.`,
			{ cause }
		);
	}
	const { instance } = await WebAssembly.instantiate(bytes, {});
	const exports = instance.exports as Partial<WasmExports>;
	if (
		!exports?.memory ||
		!exports.axolc_alloc ||
		!exports.axolc_version ||
		!exports.axolc_run ||
		!exports.axolc_compile
	) {
		throw new Error('The axolc wasm module is missing its expected exports.');
	}
	return new AxolcModule(exports as WasmExports);
}

class AxolcModule implements Axolc {
	private readonly wasm: WasmExports;

	constructor(wasm: WasmExports) {
		this.wasm = wasm;
	}

	version(): string {
		return this.readCString(this.wasm.axolc_version());
	}

	runProgram(src: string): RunResult {
		const envelope = this.callWithString(this.wasm.axolc_run, src);
		return { output: envelope.output, diagnostics: envelope.diagnostics };
	}

	compileProgram(src: string): CompileResult {
		const envelope = this.callWithString(this.wasm.axolc_compile, src);
		return { output: envelope.output, rust: envelope.rust, diagnostics: envelope.diagnostics };
	}

	private callWithString(
		entry: (src: number, srcLen: number) => number,
		src: string
	): { output: string; rust: string; diagnostics: PlaygroundDiagnostic[] } {
		const { ptr, len } = writeString(this.wasm.memory, this.wasm.axolc_alloc, src);
		const resultPtr = entry(ptr, len);
		const json = this.readCString(resultPtr);
		return parseEnvelope(json);
	}

	private readCString(ptr: number): string {
		const memory = this.wasm.memory.buffer;
		if (ptr <= 0 || ptr >= memory.byteLength) {
			throw new Error('axolc returned an out-of-bounds pointer.');
		}
		const bytes = new Uint8Array(memory);
		let end = ptr;
		while (end < bytes.length && bytes[end] !== 0) {
			end += 1;
		}
		return new TextDecoder().decode(bytes.subarray(ptr, end));
	}
}

function writeString(
	memory: WebAssembly.Memory,
	alloc: (len: number) => number,
	text: string
): { ptr: number; len: number } {
	const encoded = new TextEncoder().encode(text);
	const ptr = alloc(encoded.length);
	new Uint8Array(memory.buffer, ptr, encoded.length).set(encoded);
	return { ptr, len: encoded.length };
}

function parseEnvelope(json: string): {
	output: string;
	rust: string;
	diagnostics: PlaygroundDiagnostic[];
} {
	let raw: unknown;
	try {
		raw = JSON.parse(json);
	} catch {
		throw new Error('axolc returned a result that could not be parsed.');
	}
	const envelope = asRecord(raw);
	return {
		output: asString(envelope.output) ?? '',
		rust: asString(envelope.rust) ?? '',
		diagnostics: Array.isArray(envelope.diagnostics)
			? envelope.diagnostics.map(normalizeDiagnostic)
			: [],
	};
}

function normalizeDiagnostic(raw: unknown): PlaygroundDiagnostic {
	const row = asRecord(raw);
	const severity = asString(row.severity);
	const code = asString(row.code);
	const message = asString(row.message);
	const line = asNumber(row.line);
	const col = asNumber(row.col);
	return {
		severity:
			severity !== null && SEVERITIES.includes(severity as DiagnosticSeverity)
				? (severity as DiagnosticSeverity)
				: 'error',
		code: code,
		message: message ?? '',
		notes: Array.isArray(row.notes)
			? row.notes.filter((note): note is string => typeof note === 'string')
			: [],
		line: line !== null && line >= 1 ? line : 1,
		col: col !== null && col >= 1 ? col : 1,
	};
}

function asRecord(value: unknown): Record<string, unknown> {
	return typeof value === 'object' && value !== null ? (value as Record<string, unknown>) : {};
}

function asString(value: unknown): string | null {
	return typeof value === 'string' ? value : null;
}

function asNumber(value: unknown): number | null {
	return typeof value === 'number' && Number.isFinite(value) ? value : null;
}
