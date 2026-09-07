/*
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Server-only markdown-to-HTML renderer using marked + shiki. Used by the docs route at build/prerender time so shiki never ships to the client.
*/

import { Marked } from 'marked';
import { codeToHtml } from 'shiki';
import { slugify, extractToc, type TocItem } from '$lib/docs';

type AsyncRendererOutput = string | Promise<string>;

const marked = new Marked<string, AsyncRendererOutput>();

marked.use({
  async: true,
  renderer: {
    heading({ tokens, depth }: { tokens: any[]; depth: number }): string {
      const text: string = (this as any).parser.parseInline(tokens);
      const id = slugify(text);
      return `<h${depth} id="${id}">${text}</h${depth}>\n`;
    },
    code({ text, lang }: { text: string; lang?: string }): AsyncRendererOutput {
      const language = (lang || 'text').split(/\s+/)[0] || 'text';
      try {
        return codeToHtml(text, { lang: language, theme: 'github-dark-default' });
      } catch {
        return `<pre class="shiki"><code>${escapeHtml(text)}</code></pre>`;
      }
    },
  },
});

function escapeHtml(s: string): string {
  return s
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
    .replace(/'/g, '&#39;');
}

export async function renderMarkdown(content: string): Promise<{ html: string; toc: TocItem[] }> {
  const toc = extractToc(content);
  const out = await marked.parse(content, { async: true });
  const html = typeof out === 'string' ? out : '';
  return { html, toc };
}
