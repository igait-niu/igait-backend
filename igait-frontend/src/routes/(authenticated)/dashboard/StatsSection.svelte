<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { FileVideo, Activity, Clock } from '@lucide/svelte';

	const stats = [
		{
			label: 'Total Submissions',
			value: '12',
			description: 'All time submissions',
			icon: FileVideo
		},
		{
			label: 'Analyses Complete',
			value: '10',
			description: 'Successfully processed',
			icon: Activity
		},
		{
			label: 'In Progress',
			value: '2',
			description: 'Currently processing',
			icon: Clock
		}
	];
</script>

<section>
	<h2 class="section-heading">Your Activity</h2>
	<div class="stats-grid">
		{#each stats as stat}
			<Card.Root class="stat-card">
				<Card.Header class="stat-header">
					<Card.Title class="stat-label">{stat.label}</Card.Title>
					<svelte:component this={stat.icon} class="h-4 w-4 text-muted-foreground" />
				</Card.Header>
				<Card.Content>
					<div class="stat-value">{stat.value}</div>
					<p class="stat-description">{stat.description}</p>
				</Card.Content>
			</Card.Root>
		{/each}
	</div>
</section>

<style>
	.section-heading {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: var(--spacing-md);
	}

	.stats-grid {
		display: grid;
		gap: var(--grid-gap);
		grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
	}

	:global(.stat-card) {
		transition: all 0.2s;
	}

	:global(.stat-card:hover) {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px hsl(var(--primary) / 0.1);
	}

	:global(.stat-header) {
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: space-between;
		padding-bottom: var(--spacing-xs);
	}

	:global(.stat-label) {
		font-size: 0.875rem;
		font-weight: 500;
	}

	.stat-value {
		font-size: 2rem;
		font-weight: 700;
		line-height: 1;
		margin-bottom: var(--spacing-xs);
	}

	.stat-description {
		font-size: 0.75rem;
		color: hsl(var(--muted-foreground));
	}
</style>
