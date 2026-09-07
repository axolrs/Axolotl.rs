<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Home page hero with the project pitch and key features.
-->
<script lang="ts">
        import { Button } from '$lib/components/ui/button';
        import { Badge } from '$lib/components/ui/badge';

        const features = [
                {
                        title: 'No runtime, ever',
                        body: 'Axolotl compiles to idiomatic Rust. Production binaries have no Axolotl VM, no GC, no interpreter - just LLVM-optimized native code.'
                },
                {
                        title: 'Lua-flavored syntax',
                        body: 'If/then/end, function/end, tables, :method calls. Familiar to Lua programmers, but statically typed and Rust-grade.'
                },
                {
                        title: 'Cargo compatibility',
                        body: 'Use any crate from crates.io directly: tokio, wgpu, serde, axum, sqlx. The Cargo ecosystem IS the Axolotl ecosystem.'
                },
                {
                        title: 'Smart compiler',
                        body: 'Gills is the LSP - a rust-analyzer fork that understands both .axol and .rs files. Cross-language navigation, completion, hover, diagnostics.'
                },
                {
                        title: 'Elixir-grade iteration',
                        body: 'axol-hot-runner is the interpreter + JIT for dev mode. Sub-200ms hot reload on a 5,000-line project with 50 dependencies.'
                },
                {
                        title: 'Zero overhead',
                        body: '10 build-your-own-x ports (Git, Redis, Shell, ...) written in both Axolotl and Rust side by side. Axolotl versions are within 5% of hand-written Rust.'
                }
        ];

        const stats = [
                { value: '20,000+', label: 'Tests' },
                { value: '10', label: 'Build-your-own-x ports' },
                { value: '<200ms', label: 'Hot reload' },
                { value: 'MIT/Apache', label: 'License' }
        ];
</script>

<div class="container mx-auto max-w-6xl px-4">

        <section class="flex flex-col items-center gap-6 py-20 text-center">
                <Badge variant="secondary" class="rounded-full">0.1.0-moss</Badge>
                <h1 class="text-5xl font-extrabold tracking-tight sm:text-7xl">
                        Axolotl
                </h1>
                <p class="max-w-2xl text-balance text-lg text-muted-foreground sm:text-xl">
                        A high-level, all-purpose, memory-safe, zero-GC programming language that compiles to idiomatic Rust.
                        Lua-flavored syntax. Rust-grade semantics. Cargo ecosystem compatibility.
                </p>
                <div class="flex flex-wrap justify-center gap-3">
                        <Button href="/docs" size="lg">Read the docs</Button>
                        <Button href="/learn" size="lg" variant="outline">Learn Axolotl</Button>
                        <Button href="/benchmarks" size="lg" variant="ghost">See benchmarks</Button>
                </div>

                <div class="mt-12 grid w-full grid-cols-2 gap-4 sm:grid-cols-4">
                        {#each stats as stat}
                                <div class="rounded-lg border border-border bg-card p-4 text-center">
                                        <div class="text-2xl font-bold">{stat.value}</div>
                                        <div class="text-xs text-muted-foreground">{stat.label}</div>
                                </div>
                        {/each}
                </div>
        </section>

        <section class="py-12">
                <h2 class="mb-8 text-center text-3xl font-bold">Why Axolotl?</h2>
                <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
                        {#each features as feature}
                                <div class="rounded-lg border border-border bg-card p-6">
                                        <h3 class="mb-2 text-lg font-semibold">{feature.title}</h3>
                                        <p class="text-sm text-muted-foreground">{feature.body}</p>
                                </div>
                        {/each}
                </div>
        </section>

        <section class="py-12">
                <h2 class="mb-8 text-center text-3xl font-bold">A quick taste</h2>
                <div class="mx-auto max-w-3xl overflow-hidden rounded-lg border border-border bg-card">
                        <div class="border-b border-border bg-muted px-4 py-2 text-xs text-muted-foreground">
                                hello.axol
                        </div>
                        <pre class="overflow-x-auto p-4 text-sm"><code>-- Hello world in Axolotl
fn main()
    print("Hello, world!")

    let name = "PascalElixir"
    let health = 100
    var score = 0
    score += 10

    print("name =", name, "health =", health, "score =", score)

    if score >= 10 then
        print("high score!")
    end
end</code></pre>
                </div>
        </section>

        <section class="py-12 text-center">
                <h2 class="mb-4 text-3xl font-bold">Open source, free, forever.</h2>
                <p class="mb-6 text-muted-foreground">Dual-licensed MIT OR Apache-2.0. No premium tier. No enterprise edition.</p>
                <div class="flex justify-center gap-3">
                        <Button href="/about" variant="outline">About</Button>
                        <Button href="/docs" variant="ghost">Docs</Button>
                </div>
        </section>

</div>
