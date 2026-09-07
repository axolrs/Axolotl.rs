// Owner: PascalElixir / axolrs (GitHub org)
// File: src/lib/components/magic/word-rotate/types.ts - shared types for the rotating text components.

import type { Snippet } from "svelte";

/** Options accepted by in-view detection for the rotating text components. */
export interface UseIsInViewOptions {
	root?: Element | null;
	margin?: string;
	threshold?: number | number[];
	once?: boolean;
}

/** Context state shared between RotatingTextContainer and its RotatingText children. */
export interface RotatingTextContextType {
	activeIndex: number;
	total: number;
	vertical: boolean;
	register: () => number;
}

/** Props accepted by the RotatingTextContainer component. */
export interface RotatingTextContainerProps {
	interval?: number;
	loop?: boolean;
	hover?: boolean;
	reverse?: boolean;
	vertical?: boolean;
	class?: string;
	children: Snippet;
}

/** Props accepted by the RotatingText component. */
export interface RotatingTextProps {
	as?: "div" | "span" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6";
	splitBy?: "characters" | "words" | "none";
	textClassName?: string;
	staggerDuration?: number;
	class?: string;
	children: Snippet;
}
