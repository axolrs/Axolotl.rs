// Owner: PascalElixir / axolrs (GitHub org)
// File: src/lib/components/magic/word-rotate/use-rotating-text-context.svelte.ts - context accessor for rotating text state.

import { getContext } from "svelte";
import type { RotatingTextContextType } from "./types";

/** Context key used to share rotating text state between the container and its children. */
export const RotatingTextContextKey = Symbol("rotating-text-context");

/** Returns the rotating text context provided by the nearest RotatingTextContainer. */
export function useRotatingText(): RotatingTextContextType | undefined {
	return getContext<RotatingTextContextType>(RotatingTextContextKey);
}
