<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { FileVideo, Activity, Clock, Loader2 } from '@lucide/svelte';
	import { isJobsLoaded, type JobsState } from '$lib/hooks';
	import type { Job } from '../../../types/Job';

	type Props = {
		jobsState: JobsState;
	};

	let { jobsState }: Props = $props();

	// Calculate stats from real jobs data
	const stats = $derived.by(() => {
		if (!isJobsLoaded(jobsState)) {
			return [
				{
					label: 'Total Submissions',
					value: '---',
					description: 'Loading...',
					icon: FileVideo,
					href: '/submissions'
				},
				{
					label: 'Analyses Complete',
					value: '---',
					description: 'Loading...',
					icon: Activity,
					href: '/submissions?filter=complete'
				},
				{
					label: 'In Progress',
					value: '---',
					description: 'Loading...',
					icon: Clock,
					href: '/submissions?filter=processing'
				}
			];
		}

		const jobs = jobsState.jobs;
		const totalSubmissions = jobs.length;
		const completedJobs = jobs.filter((job: Job) => job.status.code === 'Complete').length;
		const inProgressJobs = jobs.filter(
			(job: Job) => job.status.code === 'Processing' || job.status.code === 'Submitted'
		).length;

		return [
			{
				label: 'Total Submissions',
				value: totalSubmissions.toString(),
				description: 'All time submissions',
				icon: FileVideo,
				href: '/submissions'
			},
			{
				label: 'Analyses Complete',
				value: completedJobs.toString(),
				description: 'Successfully processed',
				icon: Activity,
				href: '/submissions?filter=completed'
			},
			{
				label: 'In Progress',
				value: inProgressJobs.toString(),
				description: 'Currently processing',
				icon: Clock,
				href: '/submissions?filter=processing'
			}
		];
	});
</script>

<section>
	<h2 class="section-heading">Your Activity</h2>
	<div class="stats-grid">
		{#each stats as stat}
			<a href={stat.href} class="stat-link">
				<Card.Root class="stat-card">
					<Card.Header class="stat-header">
						<Card.Title class="stat-label">{stat.label}</Card.Title>
						{@const Icon = stat.icon}
						<Icon class="h-4 w-4 text-muted-foreground" />
					</Card.Header>
					<Card.Content>
						<div class="stat-value">{stat.value}</div>
						<p class="stat-description">{stat.description}</p>
					</Card.Content>
				</Card.Root>
			</a>
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

	.stat-link {
		text-decoration: none;
		color: inherit;
		display: block;
	}

	:global(.stat-card) {
		transition: all 0.2s;
		cursor: pointer;
		height: 100%;
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
