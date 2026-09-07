<!--
  Owner: PascalElixir / axolrs (GitHub org)
  Brief: Single doc page that renders the markdown via MarkdownViewer.
-->
<script lang="ts">
	import MarkdownViewer from '$lib/components/MarkdownViewer.svelte';
	import { Button } from '$lib/components/ui/button';

	let { data } = $props();
</script>

<div class="container mx-auto max-w-5xl px-4 py-12">
	<div class="mb-6">
		<Button href="/docs" variant="ghost" size="sm">&larr; All docs</Button>
	</div>
	<MarkdownViewer content={data.content} />
</div>
