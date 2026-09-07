<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: The in-browser Axolotl playground - run programs through wasm-compiled axolc, inspect diagnostics, and read the generated Rust.
-->
<script lang="ts">
    import { onMount } from "svelte";
    import { Badge } from "$lib/components/ui/badge";
    import { Button } from "$lib/components/ui/button";
    import { Kbd } from "$lib/components/ui/kbd";
    import * as Tabs from "$lib/components/ui/tabs";
    import * as Alert from "$lib/components/ui/alert";
    import type { EditorView } from "@codemirror/view";
    import { createAxolEditor, replaceDocument } from "./editor";
    import {
        loadAxolc,
        type Axolc,
        type PlaygroundDiagnostic,
    } from "$lib/wasm/axolc-glue";

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

    let editorHost: HTMLElement | undefined = $state();
    let compiler: Axolc | null = $state(null);
    let loadError: string | null = $state(null);
    let busy = $state(false);
    let hasResult = $state(false);
    let output = $state("");
    let rust = $state("");
    let diagnostics: PlaygroundDiagnostic[] = $state([]);
    let activeTab = $state("output");
    let editor: EditorView | null = null;

    const ready = $derived(compiler !== null);
    const version = $derived(moduleVersion(compiler));
    const errorCount = $derived(
        diagnostics.filter((d) => d.severity === "error").length,
    );
    const warningCount = $derived(
        diagnostics.filter((d) => d.severity === "warning").length,
    );

    onMount(() => {
        if (!editorHost) return;
        try {
            editor = createAxolEditor(
                editorHost,
                DEFAULT_PROGRAM,
                () => void runProgram(),
            );
        } catch (cause: unknown) {
            loadError = cause instanceof Error ? cause.message : String(cause);
        }
        loadAxolc()
            .then((module) => {
                compiler = module;
            })
            .catch((cause: unknown) => {
                loadError =
                    cause instanceof Error ? cause.message : String(cause);
            });
        return () => {
            editor?.destroy();
            editor = null;
        };
    });

    async function runProgram(): Promise<void> {
        const axolc = compiler;
        if (!axolc || !editor || busy) return;
        busy = true;
        await nextFrame();
        try {
            const result = axolc.runProgram(editor.state.doc.toString());
            output = result.output;
            diagnostics = result.diagnostics;
            rust = "";
            activeTab = "output";
            hasResult = true;
        } catch (cause: unknown) {
            loadError = cause instanceof Error ? cause.message : String(cause);
        } finally {
            busy = false;
        }
    }

    async function compileProgram(): Promise<void> {
        const axolc = compiler;
        if (!axolc || !editor || busy) return;
        busy = true;
        await nextFrame();
        try {
            const result = axolc.compileProgram(editor.state.doc.toString());
            rust = result.rust;
            diagnostics = result.diagnostics;
            output = "";
            activeTab = "rust";
            hasResult = true;
        } catch (cause: unknown) {
            loadError = cause instanceof Error ? cause.message : String(cause);
        } finally {
            busy = false;
        }
    }

    function resetProgram(): void {
        if (!editor) return;
        replaceDocument(editor, DEFAULT_PROGRAM);
        editor.focus();
    }

    function nextFrame(): Promise<void> {
        return new Promise((resolve) => setTimeout(resolve, 0));
    }

    function moduleVersion(module: Axolc | null): string | null {
        return module ? module.version() : null;
    }
</script>

<div class="container mx-auto max-w-7xl px-4 py-12">
    <header class="mb-8 flex flex-wrap items-end justify-between gap-4">
        <div>
            <h1 class="mb-2 text-4xl font-extrabold">Playground</h1>
            <p class="text-lg text-muted-foreground">
                Write Axolotl, run it, and read the Rust it compiles to -
                entirely in your browser via WebAssembly.
            </p>
        </div>
        <Badge variant="secondary" class="rounded-full">
            {#if version}axolc {version} · wasm{:else}wasm playground{/if}
        </Badge>
    </header>

    {#if loadError}
        <div class="mb-6">
            <Alert.Root variant="destructive">
                <Alert.Title>Compiler failed to load</Alert.Title>
                <Alert.Description>{loadError}</Alert.Description>
            </Alert.Root>
        </div>
    {/if}

    <div class="grid items-start gap-6 lg:grid-cols-2">
        <section
            class="overflow-hidden rounded-lg border border-border bg-card"
            aria-label="Axolotl editor"
        >
            <div
                class="flex flex-wrap items-center justify-between gap-3 border-b border-border px-4 py-3"
            >
                <div class="flex items-center gap-2">
                    <span class="font-mono text-sm font-semibold"
                        >main.axol</span
                    >
                    {#if !ready && !loadError}
                        <Badge
                            variant="secondary"
                            class="animate-pulse rounded-full text-xs"
                            >Loading compiler…</Badge
                        >
                    {/if}
                </div>
                <div class="flex items-center gap-2">
                    <Button
                        variant="outline"
                        size="sm"
                        onclick={() => void compileProgram()}
                        disabled={!ready || busy}
                    >
                        {busy ? "Working…" : "View Rust"}
                    </Button>
                    <Button
                        size="sm"
                        onclick={() => void runProgram()}
                        disabled={!ready || busy}
                    >
                        Run
                    </Button>
                    <Button
                        variant="ghost"
                        size="sm"
                        onclick={resetProgram}
                        aria-label="Reset to the default program"
                    >
                        Reset
                    </Button>
                </div>
            </div>
            <div
                class="h-[60vh] max-h-[36rem] min-h-[20rem] bg-zinc-950"
                bind:this={editorHost}
                aria-label="Axolotl source editor"
            ></div>
            <div
                class="border-t border-border px-4 py-2 text-xs text-muted-foreground"
            >
                Press <Kbd>Ctrl</Kbd>/<Kbd>Cmd</Kbd> + <Kbd>Enter</Kbd> to run the
                program.
            </div>
        </section>

        <section
            class="rounded-lg border border-border bg-card"
            aria-label="Program results"
        >
            <Tabs.Root bind:value={activeTab}>
                <div class="border-b border-border px-4 py-3">
                    <Tabs.List>
                        <Tabs.Trigger value="output">Output</Tabs.Trigger>
                        <Tabs.Trigger value="rust">Generated Rust</Tabs.Trigger>
                    </Tabs.List>
                </div>
                <Tabs.Content value="output" class="p-0">
                    <div
                        class="max-h-[26rem] min-h-[12rem] overflow-auto p-4"
                        aria-label="Program output"
                        aria-live="polite"
                    >
                        {#if hasResult}
                            {#if output}
                                <pre
                                    class="whitespace-pre-wrap break-words font-mono text-sm leading-relaxed">{output}</pre>
                            {:else}
                                <p
                                    class="font-mono text-sm text-muted-foreground"
                                >
                                    Program produced no output.
                                </p>
                            {/if}
                        {:else}
                            <p class="font-mono text-sm text-muted-foreground">
                                Run the program to see its output here.
                            </p>
                        {/if}
                    </div>
                </Tabs.Content>
                <Tabs.Content value="rust" class="p-0">
                    <div
                        class="max-h-[26rem] min-h-[12rem] overflow-auto p-4"
                        aria-label="Generated Rust source"
                    >
                        {#if rust}
                            <pre
                                class="whitespace-pre font-mono text-sm leading-relaxed">{rust}</pre>
                        {:else}
                            <p class="font-mono text-sm text-muted-foreground">
                                Press "View Rust" to compile and read the
                                generated Rust source.
                            </p>
                        {/if}
                    </div>
                </Tabs.Content>
            </Tabs.Root>

            <div class="border-t border-border">
                <div class="flex items-center justify-between gap-2 px-4 py-2">
                    <span class="text-sm font-medium">Diagnostics</span>
                    <div class="flex items-center gap-2">
                        {#if errorCount}
                            <Badge variant="destructive" class="rounded-full">
                                {errorCount}
                                {errorCount === 1 ? "error" : "errors"}
                            </Badge>
                        {/if}
                        {#if warningCount}
                            <Badge variant="secondary" class="rounded-full">
                                {warningCount}
                                {warningCount === 1 ? "warning" : "warnings"}
                            </Badge>
                        {/if}
                        {#if !errorCount && !warningCount}
                            <Badge variant="secondary" class="rounded-full"
                                >clean</Badge
                            >
                        {/if}
                    </div>
                </div>
                {#if diagnostics.length}
                    <ul
                        class="max-h-48 divide-y divide-border overflow-y-auto border-t border-border"
                    >
                        {#each diagnostics as d}
                            <li class="px-4 py-2.5">
                                <div
                                    class="flex flex-wrap items-baseline gap-x-2 gap-y-1"
                                >
                                    <span
                                        class="font-mono text-xs text-muted-foreground"
                                        >main.axol:{d.line}:{d.col}</span
                                    >
                                    <span
                                        class="text-xs font-semibold"
                                        class:text-destructive={d.severity ===
                                            "error"}
                                        class:text-amber-600={d.severity ===
                                            "warning"}
                                        class:dark:text-amber-400={d.severity ===
                                            "warning"}
                                    >
                                        {d.severity}{d.code
                                            ? ` [${d.code}]`
                                            : ""}:
                                    </span>
                                    <span class="text-sm">{d.message}</span>
                                </div>
                                {#if d.notes.length}
                                    <ul class="mt-1 space-y-0.5 pl-4">
                                        {#each d.notes as note}
                                            <li
                                                class="text-xs text-muted-foreground"
                                            >
                                                note: {note}
                                            </li>
                                        {/each}
                                    </ul>
                                {/if}
                            </li>
                        {/each}
                    </ul>
                {/if}
            </div>
        </section>
    </div>

    <footer
        class="mt-8 flex flex-wrap items-center justify-between gap-2 border-t border-border pt-6 text-sm text-muted-foreground"
    >
        <span>axolc {version ?? "…"} · wasm</span>
        <span
            >Everything compiles and runs locally - no code leaves your browser.</span
        >
    </footer>
</div>
