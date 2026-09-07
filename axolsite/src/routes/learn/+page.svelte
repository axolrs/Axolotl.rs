<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Learn index page listing tutorial chapters derived from the language bible.
-->
<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';

	const chapters = [
		{ n: 1, title: 'Hello World', desc: 'The minimal Axolotl program. Confirms the toolchain is wired correctly.' },
		{ n: 2, title: 'Comments', desc: 'Lua-style -- single and --[[ block ]] comments.' },
		{ n: 3, title: 'Modules and use', desc: 'How to import crates from Cargo and local files.' },
		{ n: 4, title: 'Variables (let and var)', desc: 'let is immutable; var is mutable. There is no mut keyword.' },
		{ n: 5, title: 'Primitive Types', desc: 'Int, UInt, Float, Double, Bool, String, Char, Byte.' },
		{ n: 6, title: 'Sized Integer / Float Types', desc: 'I8..I128, U8..U128, F32, F64 - explicit bit widths.' },
		{ n: 7, title: 'Booleans', desc: 'true and false. Logical operators are and / or / not.' },
		{ n: 8, title: 'Strings', desc: 'UTF-8 owned String and borrowed &str. ${expr} interpolation.' },
		{ n: 9, title: 'Type Inference', desc: 'The compiler deduces types from usage and initializers.' },
		{ n: 10, title: 'Type Annotations', desc: 'Explicit : Type after a binding, parameter, or return value.' },
		{ n: 11, title: 'Nullable Types', desc: 'T? is Option<T>. null is its empty value. ?. !. ??' },
		{ n: 12, title: 'Print', desc: 'print is variadic, space-separated, appends a newline.' },
		{ n: 13, title: 'Functions', desc: 'fn name(params) -> ReturnType ... end blocks.' },
		{ n: 14, title: 'Multiple Return Values', desc: 'value, err = may_fail() Lua-style.' },
		{ n: 15, title: 'Closures', desc: 'Anonymous functions with inferred captures (Fn/FnMut/FnOnce).' },
		{ n: 16, title: 'Structs', desc: 'Player = struct ... end - product types.' },
		{ n: 17, title: 'Methods', desc: 'Type.method = fn(self, ...) ... end - no impl block needed.' },
		{ n: 18, title: 'Tables and Record Literals', desc: '{ key = value, ... } - Lua-style statically typed records.' },
		{ n: 19, title: 'Enums', desc: 'Sum types. Variant can be unit, tuple, or struct.' },
		{ n: 20, title: 'Pattern Matching', desc: 'match value ... end - exhaustive destructuring with guards.' },
		{ n: 21, title: 'Interfaces (Traits)', desc: 'Drawable = interface ... end - named method contracts.' },
		{ n: 22, title: 'Implementation Blocks', desc: 'Player : Drawable - connect types to behavior.' },
		{ n: 23, title: 'Generics', desc: 'Pair = struct<T> ... end - usually inferred at call site.' },
		{ n: 24, title: 'Constants and Compile-Time Eval', desc: 'const MAX: Int = 16 - pure compile-time values.' },
		{ n: 25, title: 'If / elseif / else / end', desc: 'Conditional execution. elseif chains.' },
		{ n: 26, title: 'While / do / end', desc: 'Pre-test loops.' },
		{ n: 27, title: 'Repeat / until', desc: 'Post-test loops (Lua heritage).' },
		{ n: 28, title: 'For (Numeric and Iterator)', desc: 'for x in xs do ... end - numeric and iterator forms.' },
		{ n: 29, title: 'Break / Continue', desc: 'Loop control flow.' },
		{ n: 30, title: 'Arrays', desc: '[1, 2, 3] - Vec<T> literals.' },
		{ n: 31, title: 'Maps', desc: '{ key = value } - HashMap literals.' },
		{ n: 33, title: 'Ownership (Inferred)', desc: 'The compiler decides borrow, mut borrow, or move.' },
		{ n: 37, title: 'Result and Error Handling', desc: '! ? ?? try/catch - Rust-grade error propagation.' },
		{ n: 42, title: 'async / await', desc: 'Coroutines, tasks, spawn, channels.' },
		{ n: 71, title: 'Match Replacements', desc: 'case syntax, variant handlers, dispatch tables.' },
		{ n: 76, title: 'Pipe |>', desc: 'Elixir-style forward pipe and reverse pipe <id.' },
		{ n: 82, title: 'Comprehensions', desc: 'List, map, set comprehensions.' }
	];
</script>

<div class="container mx-auto max-w-6xl px-4 py-12">
	<h1 class="mb-2 text-4xl font-extrabold">Learn Axolotl</h1>
	<p class="mb-8 text-muted-foreground">A progressive tutorial derived from the 91-section language bible.</p>

	<div class="grid gap-3 md:grid-cols-2">
		{#each chapters as ch}
			<a href="/docs/language-reference" class="flex items-start gap-4 rounded-lg border border-border bg-card p-4 transition-colors hover:bg-accent">
				<Badge variant="secondary" class="mt-1 rounded-full text-xs">{ch.n}</Badge>
				<div>
					<h2 class="font-semibold">{ch.title}</h2>
					<p class="text-sm text-muted-foreground">{ch.desc}</p>
				</div>
			</a>
		{/each}
	</div>
</div>
