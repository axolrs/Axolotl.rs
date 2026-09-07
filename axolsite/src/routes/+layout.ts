/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Root layout load - prerenders everything by default and exposes the site config to every page.
*/

import { siteConfig, mainNav } from '$lib/site';

export const prerender = true;
export const ssr = true;
export const trailingSlash = 'never';

export async function load() {
  return {
    siteConfig,
    mainNav,
  };
}
