<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Top-level layout with navigation bar and footer for the Axolotl website.
-->
<script lang="ts">
    import "./layout.css";
    import favicon from "$lib/assets/favicon.svg";
    import { page } from "$app/stores";
    import { ModeWatcher } from "mode-watcher";
    import AnimatedThemeToggler from "$lib/components/magic/animated-theme-toggler/animated-theme-toggler.svelte";

    let { children } = $props();

    const navLinks = [
        { href: "/", label: "Home" },
        { href: "/docs", label: "Docs" },
        { href: "/learn", label: "Learn" },
        { href: "/benchmarks", label: "Benchmarks" },
        { href: "/play", label: "Play" },
        { href: "/about", label: "About" },
    ];

    const isActive = (href: string) =>
        href === "/"
            ? $page.url.pathname === "/"
            : $page.url.pathname.startsWith(href);
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>

<ModeWatcher />

<div class="flex min-h-screen flex-col bg-background text-foreground">
    <header
        class="sticky top-0 z-50 w-full border-b border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60"
    >
        <div
            class="container mx-auto flex h-14 max-w-6xl items-center justify-between px-4"
        >
            <a href="/" class="flex items-center gap-2 font-bold text-lg">
                <span class="text-xl">Axolotl</span>
                <span class="text-xs font-normal text-muted-foreground"
                    >.rs</span
                >
            </a>
            <nav class="flex items-center gap-1 text-sm">
                {#each navLinks as link}
                    <a
                        href={link.href}
                        class="rounded-md px-3 py-2 transition-colors hover:bg-accent hover:text-accent-foreground"
                        class:text-primary={isActive(link.href)}
                        class:text-muted-foreground={!isActive(link.href)}
                    >
                        {link.label}
                    </a>
                {/each}
            </nav>
        </div>
    </header>

    <main class="flex-1">
        {@render children()}
    </main>

    <footer class="border-t border-border py-8">
        <div
            class="container mx-auto max-w-6xl px-4 text-sm text-muted-foreground"
        >
            <div class="flex flex-col gap-4 md:flex-row md:justify-between">
                <div>
                    <p class="font-semibold text-foreground">Axolotl</p>
                    <p class="mt-1">
                        A high-level, memory-safe, zero-GC programming language.
                    </p>
                    <AnimatedThemeToggler />
                </div>
                <div class="flex flex-col gap-1">
                    <p>
                        Owners: <span class="font-mono">PascalElixir</span> /
                        <span class="font-mono">axolrs</span>
                    </p>
                    <p>License: MIT OR Apache-2.0</p>
                    <p>&copy; 2026 axolrs. Free, open source, forever.</p>
                </div>
            </div>
        </div>
    </footer>
</div>
