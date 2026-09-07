<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Documentation index page listing all user-facing language docs.
-->
<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';

	const docs = [
		{ slug: 'getting-started', title: 'Getting Started', description: 'Install the toolchain, create your first Axolotl project, and run it in both interpreted and compiled modes.', kind: 'Guide' },
		{ slug: 'language-reference', title: 'Language Reference', description: 'The complete Axolotl (.axol) language reference: every construct, every type, every operator with examples.', kind: 'Language' },
		{ slug: 'ownership', title: 'Ownership', description: 'How Axolotl\'s ownership system maps to Rust\'s: borrow, mut, move, and the borrow checker.', kind: 'Language' },
		{ slug: 'high-level', title: 'High-Level Surface', description: 'The enjoyable ergonomics: match replacements, let-else, pipe, named args, comprehensions, and more.', kind: 'Language' },
		{ slug: 'toolchain', title: 'Toolchain', description: 'The bucket build orchestrator, axolc compiler, axol-hot-runner interpreter, Gills LSP, and the rest of the toolchain.', kind: 'Toolchain' },
		{ slug: 'interpreter', title: 'Interpreter + JIT', description: 'The axol-hot-runner design for sub-200ms dev iteration. Tree-walk, bytecode, and Cranelift JIT modes.', kind: 'Toolchain' },
		{ slug: 'gills', title: 'Gills (LSP)', description: 'The rust-analyzer fork that understands both .axol and .rs files. Cross-language navigation, completion, hover, diagnostics.', kind: 'Toolchain' },
		{ slug: 'architecture', title: 'Architecture', description: 'The five candidate architectures, the chosen one (metadata-aware codegen to Rust), and why the others were rejected.', kind: 'Design' },
		{ slug: 'comparison', title: 'Comparison', description: 'Axolotl vs Lua, Rust, TypeScript, Nim, Zig, Mojo, Carbon. What each takes, what each refuses, why each exists.', kind: 'Design' },
		{ slug: 'examples', title: 'Examples', description: 'A collection of complete Axolotl examples: hello world, structs, enums, closures, async, traits, FFI, and more.', kind: 'Guide' },
		{ slug: 'editor-setup', title: 'Editor Setup', description: 'How to set up Zed, Neovim, and VS Code for Axolotl development with Gills (the LSP).', kind: 'Guide' },
		{ slug: 'faq', title: 'FAQ', description: 'Frequently asked questions about Axolotl: what it is, performance, ecosystem, ownership, license, toolchain, tests, benchmarks, editor support.', kind: 'Guide' },
		{ slug: 'contributing', title: 'Contributing', description: 'How to contribute to Axolotl: the workflow rule, the hard rules, the project structure, the worklog protocol, the license.', kind: 'Guide' }
	];
</script>

<div class="container mx-auto max-w-6xl px-4 py-12">
	<h1 class="mb-2 text-4xl font-extrabold">Documentation</h1>
	<p class="mb-8 text-muted-foreground">The complete Axolotl design docs and language reference.</p>

	<div class="grid gap-4 md:grid-cols-2">
		{#each docs as doc}
			<a href="/docs/{doc.slug}" class="block rounded-lg border border-border bg-card p-6 transition-colors hover:bg-accent">
				<div class="mb-2 flex items-center justify-between gap-2">
					<h2 class="text-lg font-semibold">{doc.title}</h2>
					<Badge variant="secondary" class="rounded-full text-xs">{doc.kind}</Badge>
				</div>
				<p class="text-sm text-muted-foreground">{doc.description}</p>
			</a>
		{/each}
	</div>
</div>
