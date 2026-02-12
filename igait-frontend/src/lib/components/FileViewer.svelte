<script lang="ts">
	import { Loader2, FileText, FileQuestion, Download } from '@lucide/svelte';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import JsonTree from './JsonTree.svelte';
	import type { FileEntry } from '$lib/api';

	interface Props {
		/** Files for the current stage (input or output) */
		files: FileEntry[] | undefined;
		/** Whether files are still loading */
		loading: boolean;
		/** Error message if fetch failed */
		error: string | null;
		/** Label shown in the empty state */
		label: string;
	}

	let { files, loading, error, label }: Props = $props();

	/** File content cache for text-based viewers (JSON, etc.) */
	let textCache: Record<string, { status: 'loading' | 'loaded' | 'error'; content: string; parsed?: unknown }> =
		$state({});

	/** Get the file extension from a filename */
	function getExtension(name: string): string {
		const dot = name.lastIndexOf('.');
		return dot === -1 ? '' : name.slice(dot + 1).toLowerCase();
	}

	/** Fetch text content for a file and cache it */
	async function fetchTextContent(file: FileEntry) {
		if (textCache[file.name]) return;

		textCache[file.name] = { status: 'loading', content: '' };

		try {
			const response = await fetch(file.url);
			if (!response.ok) throw new Error(`HTTP ${response.status}`);
			const text = await response.text();

			let parsed: unknown;
			try {
				parsed = JSON.parse(text);
			} catch {
				// Not valid JSON — will fall back to raw text
			}

			textCache[file.name] = { status: 'loaded', content: text, parsed };
		} catch {
			textCache[file.name] = { status: 'error', content: 'Failed to load file contents.' };
		}
	}

	/** Auto-fetch text content for JSON files when files change */
	$effect(() => {
		if (!files) return;
		for (const file of files) {
			if (getExtension(file.name) === 'json') {
				fetchTextContent(file);
			}
		}
	});
</script>

{#if loading}
	<div class="viewer-state">
		<Loader2 class="spinner" />
		<p>Loading files…</p>
	</div>
{:else if error}
	<div class="viewer-state">
		<p class="viewer-error">{error}</p>
	</div>
{:else if !files || files.length === 0}
	<div class="viewer-state">
		<p class="viewer-label">{label}</p>
		<p class="viewer-face">:3</p>
	</div>
{:else}
	<div class="file-grid">
		{#each files as file (file.name)}
			{@const ext = getExtension(file.name)}
			<Card.Root class="file-card">
				<Card.Header class="file-card-header">
					<Card.Title class="file-card-title">
						{#if ext === 'json'}
							<FileText class="file-icon file-icon--json" />
						{:else}
							<FileQuestion class="file-icon" />
						{/if}
						<span class="file-name">{file.name}</span>
					</Card.Title>
					<Card.Action>
						<Button variant="ghost" size="icon" href={file.url} target="_blank" rel="noopener">
							<Download class="h-4 w-4" />
						</Button>
					</Card.Action>
				</Card.Header>

				<Card.Content class="file-card-content">
					{#if ext === 'json'}
						{@const cached = textCache[file.name]}
						{#if !cached || cached.status === 'loading'}
							<div class="code-loading">
								<Loader2 class="spinner-sm" />
							</div>
						{:else if cached.status === 'error'}
							<pre class="code-block code-block--error">{cached.content}</pre>
						{:else if cached.parsed !== undefined}
							<JsonTree data={cached.parsed} />
						{:else}
							<pre class="code-block">{cached.content}</pre>
						{/if}
					{:else}
						<div class="placeholder-content">
							<p class="placeholder-face">:3</p>
							<p class="placeholder-ext">.{ext} viewer not yet implemented</p>
						</div>
					{/if}
				</Card.Content>
			</Card.Root>
		{/each}
	</div>
{/if}

<style>
	/* ── States ───────────────────────────────────────────── */

	.viewer-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 2rem;
		text-align: center;
		color: hsl(var(--muted-foreground));
		gap: 0.5rem;
	}

	.viewer-state p {
		margin: 0;
	}

	.viewer-label {
		font-size: 0.875rem;
	}

	.viewer-face {
		font-size: 2rem;
		opacity: 0.6;
	}

	.viewer-error {
		font-size: 0.8125rem;
		color: hsl(var(--destructive));
	}

	:global(.spinner-sm) {
		width: 1rem;
		height: 1rem;
		animation: spin 1s linear infinite;
	}

	/* ── Grid ─────────────────────────────────────────────── */

	.file-grid {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 1rem;
	}

	/* ── Card overrides ───────────────────────────────────── */

	:global(.file-card) {
		overflow: hidden;
	}

	:global(.file-card-header) {
		flex-direction: row;
		align-items: center;
		gap: 0.5rem;
		padding: 0.625rem 0.75rem !important;
	}

	:global(.file-card-title) {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8125rem !important;
		font-weight: 600;
	}

	:global(.file-card-content) {
		padding: 0 !important;
	}

	/* ── Icons ─────────────────────────────────────────────── */

	:global(.file-icon) {
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
		color: hsl(var(--muted-foreground));
	}

	:global(.file-icon--json) {
		color: hsl(var(--primary));
	}

	.file-name {
		font-family: ui-monospace, monospace;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	/* ── Code viewer ──────────────────────────────────────── */

	.code-loading {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1.5rem;
	}

	.code-block {
		font-family: ui-monospace, monospace;
		font-size: 0.6875rem;
		line-height: 1.6;
		white-space: pre-wrap;
		word-break: break-word;
		background: hsl(var(--muted) / 0.4);
		padding: 0.75rem 1rem;
		margin: 0;
		max-height: 400px;
		overflow-y: auto;
	}

	.code-block--error {
		color: hsl(var(--destructive));
		background: hsl(var(--destructive) / 0.06);
	}

	/* ── Placeholder ──────────────────────────────────────── */

	.placeholder-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 2rem 1rem;
		text-align: center;
		color: hsl(var(--muted-foreground));
		gap: 0.25rem;
	}

	.placeholder-face {
		font-size: 1.5rem;
		opacity: 0.6;
		margin: 0;
	}

	.placeholder-ext {
		font-size: 0.75rem;
		margin: 0;
		opacity: 0.7;
	}
</style>
