/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Loader for /learn - exposes the chaptered tour of the language derived from PROGRAMMINGLANGUAGEBIBLE sections.
*/

import { learnSections, totalChapters } from '$lib/learn';

export const prerender = true;

export function load() {
  return {
    sections: learnSections,
    totalChapters: totalChapters(),
  };
}
