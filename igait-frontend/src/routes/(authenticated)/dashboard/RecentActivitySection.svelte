<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { ArrowRight, Activity, Upload, FileVideo, Loader2 } from '@lucide/svelte';
	import { isJobsLoaded, isJobsLoading, type JobsState } from '$lib/hooks';
	import type { Job } from '../../../types/Job';

	type Props = {
		jobsState: JobsState;
	};

	let { jobsState }: Props = $props();

	// Get recent activity from jobs (last 3)
	const recentActivity = $derived.by(() => {
		if (!isJobsLoaded(jobsState)) {
			return [];
		}

		return jobsState.jobs.slice(0, 3).map((job: Job, index: number) => {
			const date = new Date(job.timestamp * 1000);
			const isCompleted = job.status.code === 'Complete';
			const isError = job.status.code.includes('Error') || job.status.code.includes('Failed');
			
			return {
				id: index,
				type: 'submission',
				status: isCompleted ? 'completed' : isError ? 'error' : 'processing',
				date: date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' }),
				description: job.status.value
			};
		});
	});
</script>

<section>
	<div class="recent-header">
		<h2 class="section-heading">Recent Activity</h2>
		<Button variant="ghost" size="sm" href="/submissions">
			View All
			<ArrowRight class="ml-2 h-4 w-4" />
		</Button>
	</div>
	
	<Card.Root>
		<Card.Content class="p-0">
			{#if isJobsLoading(jobsState)}
				<div class="empty-state">
					<Loader2 class="empty-icon animate-spin" />
					<h3 class="empty-title">Loading activity...</h3>
					<p class="empty-description">
						Fetching your recent submissions
					</p>
				</div>
			{:else if recentActivity.length === 0}
				<div class="empty-state">
					<Activity class="empty-icon" />
					<h3 class="empty-title">No activity yet</h3>
					<p class="empty-description">
						Submit your first walking video to get started!
					</p>
					<Button class="empty-button" href="/submit">
						<Upload class="mr-2 h-4 w-4" />
						New Submission
					</Button>
				</div>
			{:else}
				<div class="activity-list">
					{#each recentActivity as activity}
						<div class="activity-item">
							<div class="activity-content">
								<div class="activity-icon">
									<FileVideo class="h-5 w-5 text-muted-foreground" />
								</div>
								<div>
									<p class="activity-title">{activity.description}</p>
									<p class="activity-date">{activity.date}</p>
								</div>
							</div>
							<Badge 
								variant={activity.status === 'completed' ? 'default' : activity.status === 'error' ? 'destructive' : 'secondary'}
							>
								{activity.status === 'completed' ? 'Complete' : activity.status === 'error' ? 'Error' : 'Processing'}
							</Badge>
						</div>
					{/each}
				</div>
			{/if}
		</Card.Content>
	</Card.Root>
</section>

<style>
	.section-heading {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: var(--spacing-md);
	}

	.recent-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-bottom: var(--spacing-md);
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--spacing-2xl);
		text-align: center;
	}

	:global(.empty-icon) {
		height: 3rem;
		width: 3rem;
		color: hsl(var(--muted-foreground));
		margin-bottom: var(--spacing-md);
	}

	.empty-title {
		font-size: 1.125rem;
		font-weight: 600;
		margin-bottom: var(--spacing-xs);
	}

	.empty-description {
		color: hsl(var(--muted-foreground));
		margin-bottom: var(--spacing-md);
	}

	.activity-list {
		display: flex;
		flex-direction: column;
	}

	.activity-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: var(--spacing-md);
		border-bottom: 1px solid hsl(var(--border));
	}

	.activity-item:last-child {
		border-bottom: none;
	}

	.activity-content {
		display: flex;
		align-items: center;
		gap: var(--spacing-md);
	}

	.activity-icon {
		display: flex;
		height: 2.5rem;
		width: 2.5rem;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-md);
		background-color: hsl(var(--muted) / 0.5);
	}

	.activity-title {
		font-weight: 500;
		margin-bottom: 0.125rem;
	}

	.activity-date {
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
	}
</style>
