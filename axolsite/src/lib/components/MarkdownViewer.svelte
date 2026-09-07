<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Markdown viewer component using marked + shiki for syntax highlighting.
-->
<script lang="ts">
	import { onMount } from 'svelte';
	import { marked, type Tokens } from 'marked';
	import { createHighlighter, type Highlighter } from 'shiki';

	let { content }: { content: string } = $props();

	let html = $state('');
	let toc = $state<{ slug: string; text: string; level: number }[]>([]);

	onMount(async () => {
		const highlighter: Highlighter = await createHighlighter({
			themes: ['github-dark', 'github-light'],
			langs: ['rust', 'typescript', 'javascript', 'bash', 'json', 'markdown', 'python', 'lua']
		});

		const renderer = {
			code(token: Tokens.Code): string | false {
				try {
					return highlighter.codeToHtml(token.text, {
						lang: token.lang || 'rust',
						themes: { dark: 'github-dark', light: 'github-light' }
					});
				} catch {
					return `<pre><code>${token.text}</code></pre>`;
				}
			},
			heading(token: Tokens.Heading): string | false {
				const text = token.text.replace(/<[^>]+>/g, '');
				const slug = text.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '');
				toc.push({ slug, text, level: token.depth });
				return `<h${token.depth} id="${slug}">${token.text}</h${token.depth}>`;
			}
		};

		marked.use({ renderer });

		html = marked.parse(content) as string;
	});
</script>

<div class="grid gap-8 md:grid-cols-[1fr_220px]">
	<article class="prose prose-lg max-w-none dark:prose-invert [&_pre]:!bg-muted [&_pre]:rounded-lg [&_pre]:p-4 [&_code]:!text-sm [&_pre]:!bg-transparent">
		{@html html}
	</article>

	<aside class="hidden md:block">
		{#if toc.length > 0}
			<div class="sticky top-20">
				<h3 class="mb-3 text-sm font-semibold">On this page</h3>
				<ul class="space-y-1 text-sm">
					{#each toc as item}
						<li>
							<a
								href="#{item.slug}"
								class="text-muted-foreground hover:text-primary"
								class:pl-4={item.level === 3}
							>{item.text}</a>
						</li>
					{/each}
				</ul>
			</div>
		{/if}
	</aside>
</div>
