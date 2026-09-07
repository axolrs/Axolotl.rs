<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Cycles an active index across RotatingText children on a timed interval.
-->
<script lang="ts">
        import { onDestroy, onMount, setContext } from "svelte";
        import { cn } from "$lib/utils";
        import { RotatingTextContextKey } from "./use-rotating-text-context.svelte.ts";
        import type { RotatingTextContainerProps, RotatingTextContextType } from "./types";

        let {
                interval = 5000,
                loop = true,
                hover = false,
                reverse = false,
                vertical = false,
                class: className,
                children,
        }: RotatingTextContainerProps = $props();

        let activeIndex = $state(0);
        let total = $state(0);
        let hovering = $state(false);
        let timer: ReturnType<typeof setInterval> | undefined;

        /** Registers a RotatingText child and returns its rotation ordinal. */
        function register() {
                let ordinal = total;
                total += 1;
                return ordinal;
        }

        /** Advances the active index, wrapping when looping and stopping at the configured end. */
        function advance() {
                if (total <= 1 || (hover && hovering)) {
                        return;
                }

                let atEnd = reverse ? activeIndex === 0 : activeIndex === total - 1;

                if (!loop && atEnd) {
                        if (timer) {
                                clearInterval(timer);
                                timer = undefined;
                        }
                        return;
                }

                if (reverse) {
                        activeIndex = activeIndex === 0 ? total - 1 : activeIndex - 1;
                } else {
                        activeIndex = activeIndex === total - 1 ? 0 : activeIndex + 1;
                }
        }

        let context: RotatingTextContextType = {
                get activeIndex() {
                        return activeIndex;
                },
                get total() {
                        return total;
                },
                get vertical() {
                        return vertical;
                },
                register,
        };

        setContext(RotatingTextContextKey, context);

        onMount(() => {
                timer = setInterval(advance, interval);
        });

        onDestroy(() => {
                if (timer) {
                        clearInterval(timer);
                }
        });
</script>

<span
        class={cn("inline-block", vertical && "align-top", className)}
        role="presentation"
        onpointerenter={hover ? () => (hovering = true) : undefined}
        onpointerleave={hover ? () => (hovering = false) : undefined}
>
        {@render children()}
</span>
