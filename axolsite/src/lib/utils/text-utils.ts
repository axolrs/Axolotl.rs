// Owner: PascalElixir / axolrs (GitHub org)
// File: src/lib/utils/text-utils.ts - shared text segmentation helpers for animated text components.

export type TextToken =
	| {
			kind: "text";
			value: string;
	  }
	| {
			kind: "whitespace";
			value: string;
	  };

/** Creates an Intl.Segmenter for the given granularity, or null when unsupported. */
function createSegmenter(granularity: Intl.SegmenterOptions["granularity"]) {
	if (typeof Intl === "undefined" || !("Segmenter" in Intl)) {
		return null;
	}

	return new Intl.Segmenter(undefined, { granularity });
}

/** Normalizes line endings in the given source text so downstream splitting is consistent. */
export function normalizeSourceText(value: string | null | undefined) {
	return value?.replace(/\r\n?/g, "\n") ?? "";
}

/** Reads a DOM node's text content and returns it with normalized line endings. */
export function readNormalizedTextContent(node: Node | null | undefined) {
	return normalizeSourceText(node?.textContent ?? "");
}

/** Splits a string into an array of single grapheme clusters. */
export function splitGraphemes(value: string) {
	let segmenter = createSegmenter("grapheme");

	if (!segmenter) {
		return Array.from(value);
	}

	return Array.from(segmenter.segment(value), ({ segment }) => segment);
}

/** Segments a string into text and whitespace tokens, preserving original spacing. */
export function segmentText(value: string): TextToken[] {
	if (!value) {
		return [];
	}

	let segmenter = createSegmenter("word");

	if (!segmenter) {
		return (value.match(/\S+|\s+/g) ?? []).map((segment) =>
			/^\s+$/.test(segment)
				? {
						kind: "whitespace",
						value: segment,
					}
				: {
						kind: "text",
						value: segment,
					}
		);
	}

	let tokens: TextToken[] = [];
	let currentText = "";

	for (let { segment } of segmenter.segment(value)) {
		if (/^\s+$/.test(segment)) {
			if (currentText) {
				tokens.push({
					kind: "text",
					value: currentText,
				});
				currentText = "";
			}

			tokens.push({
				kind: "whitespace",
				value: segment,
			});
			continue;
		}

		currentText += segment;
	}

	if (currentText) {
		tokens.push({
			kind: "text",
			value: currentText,
		});
	}

	return tokens;
}
