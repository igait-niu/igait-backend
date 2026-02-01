<script lang="ts">
	import { Clock } from '@lucide/svelte';
	import type { QueueItem, FinalizeQueueItem } from '$lib/hooks';

	interface Props {
		item: QueueItem | FinalizeQueueItem;
	}

	let { item }: Props = $props();

	function getRelativeTime(timestamp: number): string {
		const seconds = Math.floor((Date.now() - timestamp) / 1000);
		if (seconds < 60) return `${seconds}s ago`;
		const minutes = Math.floor(seconds / 60);
		if (minutes < 60) return `${minutes}m ago`;
		const hours = Math.floor(minutes / 60);
		return `${hours}h ago`;
	}

	function formatJobId(jobId: string): string {
		const parts = jobId.split('_');
		if (parts.length >= 2) {
			const index = parts[parts.length - 1];
			return `#${index}`;
		}
		return jobId.slice(0, 8);
	}
</script>

<div class="job-item" class:job-item--claimed={item.claimed_by}>
	<div class="job-row">
		<span class="job-id" title={item.job_id}>{formatJobId(item.job_id)}</span>
		{#if item.claimed_by}
			<span class="job-status">⚡</span>
		{/if}
		<span class="job-time">
			<Clock class="time-icon" />
			{getRelativeTime(item.enqueued_at)}
		</span>
	</div>
</div>

<style>
	.job-item {
		background: hsl(var(--muted) / 0.4);
		border-radius: 0.25rem;
		padding: 0.25rem 0.5rem;
		font-size: 0.6875rem;
	}

	.job-item--claimed {
		background: hsl(var(--primary) / 0.12);
	}

	.job-row {
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	.job-id {
		font-family: ui-monospace, monospace;
		font-weight: 500;
		color: hsl(var(--foreground));
	}

	.job-status {
		font-size: 0.625rem;
	}

	.job-time {
		display: flex;
		align-items: center;
		gap: 0.125rem;
		color: hsl(var(--muted-foreground));
		font-size: 0.625rem;
		margin-left: auto;
	}

	:global(.time-icon) {
		width: 0.625rem;
		height: 0.625rem;
	}
</style>
