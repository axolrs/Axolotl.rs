// Owner: PascalElixir / axolrs (GitHub org)
// File: src/lib/components/magic/word-rotate/index.ts - public exports for the word rotate components.

export { default as WordRotate } from "./word-rotate.svelte";
export { default as RotatingTextContainer } from "./rotating-text-container.svelte";
export { default as RotatingText } from "./rotating-text.svelte";
export { useRotatingText } from "./use-rotating-text-context.svelte.ts";

export type {
	RotatingTextContextType,
	UseIsInViewOptions,
	RotatingTextContainerProps,
	RotatingTextProps,
} from "./types";
