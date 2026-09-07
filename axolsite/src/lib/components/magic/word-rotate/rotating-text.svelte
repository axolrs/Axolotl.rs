<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Renders one rotating text segment that animates in when its ordinal becomes active.
-->
<script lang="ts">
	import { onMount } from "svelte";
	import { fly } from "svelte/transition";
	import { cn } from "$lib/utils";
	import { readNormalizedTextContent, segmentText, splitGraphemes } from "$lib/utils/text-utils";
	import { useRotatingText } from "./use-rotating-text-context.svelte.ts";
	import type { RotatingTextProps } from "./types";

	type RotatingToken = {
		id: string;
		value: string;
		whitespace: boolean;
	};

	let {
		as = "span",
		splitBy = "words",
		textClassName,
		staggerDuration = 0.05,
		class: className,
		children,
	}: RotatingTextProps = $props();

	let context = useRotatingText();
	let ordinal = $state(0);
	let sourceElement = $state<HTMLSpanElement | null>(null);

	let isActive = $derived(context ? context.activeIndex === ordinal : true);
	let vertical = $derived(context?.vertical ?? false);

	let tokens = $derived.by<RotatingToken[]>(() => {
		let text = readNormalizedTextContent(sourceElement);

		if (!text || splitBy === "none") {
			return text ? [{ id: "text", value: text, whitespace: false }] : [];
		}

		if (splitBy === "characters") {
			return splitGraphemes(text).map((value, index) => ({
				id: `char-${index}`,
				value,
				whitespace: /^\s+$/.test(value),
			}));
		}

		return segmentText(text).map((token, index) => ({
			id: `word-${index}`,
			value: token.value,
			whitespace: token.kind === "whitespace",
		}));
	});

	onMount(() => {
		ordinal = context?.register() ?? 0;
	});
</script>

<svelte:element this={as} class={cn("inline-block", className)}>
	<span bind:this={sourceElement} class="sr-only">
		{@render children()}
	</span>

	{#if isActive}
		{#each tokens as token, index (token.id)}
			{#if token.whitespace}
				<span aria-hidden="true" class="whitespace-nowrap">{token.value}</span>
			{:else}
				<span
					aria-hidden="true"
					class={cn("inline-block", splitBy === "words" && "whitespace-nowrap", textClassName)}
					in:fly={{ x: vertical ? 0 : 12, y: vertical ? 12 : 0, duration: 240, delay: index * staggerDuration * 1000 }}
				>
					{token.value}
				</span>
			{/if}
		{/each}
	{/if}
</svelte:element>
