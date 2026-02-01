<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import type { QueueItem, FinalizeQueueItem } from '$lib/hooks';
	import QueueJobItem from './QueueJobItem.svelte';

	interface Props {
		name: string;
		description: string;
		colorClass: string;
		items: (QueueItem | FinalizeQueueItem)[];
	}

	let { name, description, colorClass, items }: Props = $props();

	const isEmpty = $derived(items.length === 0);
</script>

<div class="stage-card" class:stage-card--empty={isEmpty}>
	<div class="stage-header">
		<div class="stage-indicator {colorClass}"></div>
		<div class="stage-info">
			<span class="stage-name">{name}</span>
			<span class="stage-desc">{description}</span>
		</div>
		<Badge variant={isEmpty ? 'outline' : 'default'} class="stage-badge">
			{items.length}
		</Badge>
	</div>
	
	{#if items.length > 0}
		<div class="stage-jobs">
			{#each items as item}
				<QueueJobItem {item} />
			{/each}
		</div>
	{/if}
</div>

<style>
	.stage-card {
		background: hsl(var(--card));
		border: 1px solid hsl(var(--border));
		border-radius: 0.5rem;
		padding: 0.75rem;
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		transition: border-color 0.15s ease, box-shadow 0.15s ease;
	}

	.stage-card:hover {
		border-color: hsl(var(--border) / 0.8);
		box-shadow: 0 2px 8px hsl(var(--foreground) / 0.04);
	}

	.stage-card--empty {
		opacity: 0.7;
	}

	.stage-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.stage-indicator {
		width: 8px;
		height: 8px;
		border-radius: 50%;
		flex-shrink: 0;
	}

	.stage-info {
		flex: 1;
		display: flex;
		flex-direction: column;
		min-width: 0;
		gap: 0.0625rem;
	}

	.stage-name {
		font-size: 0.8125rem;
		font-weight: 600;
		line-height: 1.2;
	}

	.stage-desc {
		font-size: 0.6875rem;
		color: hsl(var(--muted-foreground));
		line-height: 1.2;
	}

	:global(.stage-badge) {
		font-size: 0.6875rem;
		padding: 0.125rem 0.375rem;
		height: auto;
	}

	.stage-jobs {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
		max-height: 180px;
		overflow-y: auto;
		padding-top: 0.25rem;
		border-top: 1px solid hsl(var(--border) / 0.5);
	}

	/* Color classes for stage indicators */
	:global(.bg-blue-500) { background-color: #3b82f6; }
	:global(.bg-cyan-500) { background-color: #06b6d4; }
	:global(.bg-teal-500) { background-color: #14b8a6; }
	:global(.bg-green-500) { background-color: #22c55e; }
	:global(.bg-lime-500) { background-color: #84cc16; }
	:global(.bg-yellow-500) { background-color: #eab308; }
	:global(.bg-amber-500) { background-color: #f59e0b; }
</style>
