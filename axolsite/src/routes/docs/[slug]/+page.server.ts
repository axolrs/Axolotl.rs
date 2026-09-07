/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Server-side loader for /docs/[slug]. Reads markdown from static/docs/ via import.meta.glob, prerenders to HTML with marked + shiki, returns content + html + toc + meta.
*/

import { error } from '@sveltejs/kit';
import { renderMarkdown } from '$lib/server/markdown';
import { docsList, getDocMeta } from '$lib/docs';

const docModules = import.meta.glob('/static/docs/*.md', {
  eager: true,
  query: '?raw',
  import: 'default',
}) as Record<string, string>;

export const prerender = true;

export const entries = () => docsList.map((d) => ({ slug: d.slug }));

export async function load({ params }: { params: { slug: string } }) {
  const slug = params.slug;
  const meta = getDocMeta(slug);
  if (!meta) throw error(404, `Unknown document: ${slug}`);

  const path = `/static/docs/${meta.filename}`;
  const content = docModules[path];
  if (content == null) throw error(404, `Document not found on disk: ${meta.filename}`);

  const { html, toc } = await renderMarkdown(content);

  return {
    slug,
    meta,
    content,
    html,
    toc,
  };
}
