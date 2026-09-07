<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: About page with license, owners, contributing links.
-->
<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
</script>

<div class="container mx-auto max-w-3xl px-4 py-12">
	<h1 class="mb-2 text-4xl font-extrabold">About Axolotl</h1>
	<p class="mb-8 text-muted-foreground">A high-level, all-purpose, memory-safe, zero-GC programming language.</p>

	<section class="mb-8">
		<h2 class="mb-2 text-2xl font-bold">Owners</h2>
		<p class="text-muted-foreground">The only handles that appear in code, commits, generated docs, and public surfaces are:</p>
		<ul class="mt-3 list-disc pl-6">
			<li><code class="rounded bg-muted px-1.5 py-0.5">PascalElixir</code></li>
			<li>the <code class="rounded bg-muted px-1.5 py-0.5">axolrs</code> GitHub organization</li>
		</ul>
	</section>

	<section class="mb-8">
		<h2 class="mb-2 text-2xl font-bold">License</h2>
		<p class="mb-3 text-muted-foreground">Axolotl is dual-licensed <strong>MIT OR Apache-2.0</strong> at the user's preference. Open source, free, forever.</p>
		<div class="flex gap-2">
			<Badge variant="secondary" class="rounded-full">MIT</Badge>
			<Badge variant="secondary" class="rounded-full">Apache-2.0</Badge>
		</div>
		<p class="mt-3 text-sm text-muted-foreground">There is no premium tier. There is no enterprise edition. There is one release, one license, one repo.</p>
	</section>

	<section class="mb-8">
		<h2 class="mb-2 text-2xl font-bold">Toolchain</h2>
		<ul class="list-disc pl-6 text-muted-foreground">
			<li><strong>axolc</strong> &mdash; the Axolotl to Rust compiler.</li>
			<li><strong>axol-hot-runner</strong> &mdash; the interpreter + JIT for dev mode.</li>
			<li><strong>axol-analyzer (Gills)</strong> &mdash; the LSP server.</li>
			<li><strong>bucket</strong> &mdash; the build orchestrator.</li>
			<li><strong>Shed</strong> &mdash; the formatter.</li>
			<li><strong>Neoten</strong> &mdash; the linter.</li>
			<li><strong>Regrow</strong> &mdash; the auto-fixer.</li>
			<li><strong>Pond</strong> &mdash; the global build cache.</li>
		</ul>
	</section>

	<section class="mb-8">
		<h2 class="mb-2 text-2xl font-bold">Contributing</h2>
		<p class="text-muted-foreground">Read the <a href="/docs/contributing" class="text-primary underline">contributing guide</a> for the rules and the workflow.</p>
	</section>

	<section class="mb-8">
		<h2 class="mb-2 text-2xl font-bold">Rust edition</h2>
		<p class="text-muted-foreground">The Cargo workspace uses Rust edition <strong>2024</strong>. The calendar year context is <strong>2026</strong>.</p>
	</section>

	<section class="mb-8">
		<h2 class="mb-2 text-2xl font-bold">Code of Conduct</h2>
		<p class="text-muted-foreground">All contributors follow the <a href="/docs/faq" class="text-primary underline">code of conduct</a>.</p>
	</section>

	<div class="mt-12 flex gap-3">
		<Button href="/docs" size="lg">Read the docs</Button>
		<Button href="/learn" size="lg" variant="outline">Learn</Button>
	</div>
</div>
