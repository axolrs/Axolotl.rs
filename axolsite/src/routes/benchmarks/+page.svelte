<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Benchmarks page listing the 10 build-your-own-x ports.
-->
<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';

	const projects = [
		{ name: 'Git', desc: 'Version control. Exercises parsing, hashing, object storage, network protocols, and a CLI.' },
		{ name: 'Database', desc: 'A simple key-value store. Async, FFI for the storage engine, error handling, Option/Result ergonomics.' },
		{ name: 'Redis', desc: 'Network protocol parsing, in-memory data structures, async. Tests Vec, Map, String, Result, pattern matching.' },
		{ name: 'Shell', desc: 'A POSIX shell. String parsing, process spawning (FFI), pipelines, signals.' },
		{ name: 'Web Server', desc: 'An HTTP/1.1 server. Async, networking, the foreign-block machinery, TcpListener.' },
		{ name: 'Text Editor', desc: 'A vim-like editor. Terminal I/O, key bindings, file editing, undo/redo, collection ergonomics.' },
		{ name: 'Programming Language', desc: 'A tree-walking interpreter for a small Lisp. Exercises Axolotl\'s own surface.' },
		{ name: 'Regex Engine', desc: 'A backtracking / NFA / DFA regex engine. Pattern matching, sum types, recursion, tail calls.' },
		{ name: 'Docker', desc: 'A minimal container runtime using Linux namespaces and cgroups. Unsafe, FFI to libc, process management.' },
		{ name: 'Emulator / VM', desc: 'A CHIP-8 VM. u8/u16/u32/u64, bitwise operators, unsafe, FFI.' }
	];

	const criterion = 'axol_time / rust_time &le; 1.05';
</script>

<div class="container mx-auto max-w-6xl px-4 py-12">
	<h1 class="mb-2 text-4xl font-extrabold">Benchmarks</h1>
	<p class="mb-4 text-muted-foreground">10 build-your-own-x ports, written in both Axolotl and Rust, side by side.</p>

	<div class="mb-8 rounded-lg border border-border bg-card p-4">
		<h2 class="mb-2 font-semibold">Acceptance criterion</h2>
		<code class="rounded bg-muted px-2 py-1 text-sm">{criterion}</code>
		<p class="mt-2 text-sm text-muted-foreground">The 5% tolerance is well within LLVM\'s own variance. If the Axolotl version is more than 5% slower, the Axolotl port is wrong &mdash; the generated Rust is not idiomatic, or the Axolotl surface is being used suboptimally.</p>
	</div>

	<div class="grid gap-4 md:grid-cols-2">
		{#each projects as p, i}
			<div class="rounded-lg border border-border bg-card p-6">
				<div class="mb-3 flex items-center justify-between">
					<h2 class="text-lg font-semibold">{p.name}</h2>
					<Badge variant="secondary" class="rounded-full text-xs">#{i + 1}</Badge>
				</div>
				<p class="text-sm text-muted-foreground">{p.desc}</p>
			</div>
		{/each}
	</div>

	<div class="w-full flex flex-col items-center justify-center mt-12 p-4 bg-card rounded">
	    <h3 class="font-bold">NOTE: there's more in the source code</h3>
	</div>
</div>
