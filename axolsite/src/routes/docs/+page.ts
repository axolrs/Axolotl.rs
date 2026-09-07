/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Loader for /docs index - exposes the categorized docs list so the page can render cards without shipping the catalog to the client twice.
*/

import { docsList, docCategories, docsByCategory, getDocMeta } from '$lib/docs';

export const prerender = true;

export function load() {
  return {
    docsList,
    docCategories,
    byCategory: docsByCategory(),
    featured: getDocMeta('PROGRAMMINGLANGUAGEBIBLE') ?? null,
  };
}
