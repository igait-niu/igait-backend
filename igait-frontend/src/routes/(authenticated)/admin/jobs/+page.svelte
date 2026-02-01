<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { subscribeToAllJobs, type AllJobsState, type AdminJob } from '$lib/hooks';
	import { JobsDataTable } from '$lib/components/jobs';
	import AdminLoadingState from '../AdminLoadingState.svelte';
	import AdminErrorState from '../AdminErrorState.svelte';
	import JobDetailsDialog from '../../submissions/JobDetailsDialog.svelte';
	import type { Job } from '../../../../types/Job';

	let jobsState: AllJobsState = $state({ loading: true, jobs: [] });
	let unsubscribe: (() => void) | undefined;
	let selectedJob: (Job & { id: string }) | null = $state(null);

	// We need a "fake" uid for the data table since admin sees all users
	// The jobs already have proper IDs set from the hook
	const adminUid = $derived(jobsState.jobs[0]?.id.split('_')[0] ?? 'admin');

	// Transform jobs to have id already set (they do from the hook)
	const jobsForTable = $derived(jobsState.jobs.map(job => {
		// Strip the id field for the table since it adds its own
		const { id, ...rest } = job;
		return { ...rest, _originalId: id } as Job & { _originalId: string };
	}));

	function handleViewDetails(job: Job & { id: string }) {
		selectedJob = job;
	}

	function handleCloseDetails() {
		selectedJob = null;
	}

	onMount(() => {
		unsubscribe = subscribeToAllJobs((newState) => {
			jobsState = newState;
		});
	});

	onDestroy(() => {
		unsubscribe?.();
	});
</script>

<svelte:head>
	<title>Job Overview - Admin - iGait</title>
</svelte:head>

<div class="jobs-page">
	<header class="page-header">
		<h1 class="page-title">Job Overview</h1>
		<p class="page-description">All jobs across the system</p>
	</header>

	{#if jobsState.loading}
		<AdminLoadingState message="Loading jobs..." />
	{:else if jobsState.error}
		<AdminErrorState message={jobsState.error} />
	{:else if jobsState.jobs.length === 0}
		<div class="empty-state">
			<p>No jobs found in the system</p>
		</div>
	{:else}
		<JobsDataTable 
			data={jobsState.jobs} 
			uid=""
			showEmail={true}
			onViewDetails={handleViewDetails}
		/>
	{/if}
</div>

{#if selectedJob}
	<JobDetailsDialog job={selectedJob} onClose={handleCloseDetails} />
{/if}

<style>
	.jobs-page {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.page-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.page-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: hsl(var(--foreground));
		margin: 0;
	}

	.page-description {
		font-size: 0.8125rem;
		color: hsl(var(--muted-foreground));
		margin: 0;
	}

	.empty-state {
		text-align: center;
		padding: 2rem;
		color: hsl(var(--muted-foreground));
		font-size: 0.875rem;
	}
</style>
