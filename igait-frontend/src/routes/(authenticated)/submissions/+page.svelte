<script lang="ts">
	import { onDestroy } from 'svelte';
	import { page } from '$app/stores';
	import { getUser, subscribeToJobs, isJobsLoading, isJobsError, isJobsLoaded, type JobsState } from '$lib/hooks';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { FileVideo, Loader2, AlertCircle } from '@lucide/svelte';

	import EmptyState from './EmptyState.svelte';
	import SupportCard from './SupportCard.svelte';
	import DataTable from './data-table.svelte';

	const user = getUser();

	let jobsState: JobsState = $state({ status: 'loading' });

	const unsubscribe = subscribeToJobs(user.uid, (state) => {
		jobsState = state;
		// Debug: Log the jobs state
		if (state.status === 'loaded') {
			console.log('[Submissions] Jobs loaded:', state.jobs.length, 'jobs');
			console.log('[Submissions] Jobs data:', state.jobs);
		}
	});

	onDestroy(() => {
		unsubscribe();
	});

	// Get status filter from URL query param (?filter=completed, ?filter=processing, ?filter=error)
	const statusFilter = $derived($page.url.searchParams.get('filter') || undefined);
</script>

<svelte:head>
	<title>Submissions - iGait</title>
</svelte:head>

<div class="submissions-page">
	<section class="page-header">
		<h1 class="page-header__title">Submissions</h1>
		<p class="page-header__description">
			View and manage your gait analysis submissions
		</p>
	</section>

	<Card.Root>
		<Card.Header>
			<Card.Title>Your Submissions</Card.Title>
			<Card.Description>
				Real-time view of all your gait analysis submissions
			</Card.Description>
		</Card.Header>
		<Card.Content>
			{#if isJobsLoading(jobsState)}
				<EmptyState
					icon={Loader2}
					title="Loading submissions..."
					description="Fetching your gait analysis submissions"
					variant="loading"
				/>
			{:else if isJobsError(jobsState)}
				<EmptyState
					icon={AlertCircle}
					title="Unable to load submissions"
					description={jobsState.error}
					variant="error"
				/>
			{:else if isJobsLoaded(jobsState) && jobsState.jobs.length === 0}
				<EmptyState
					icon={FileVideo}
					title="No submissions yet"
					description="You haven't made any gait analysis submissions yet."
				>
					<Button href="/submit">Make Your First Submission</Button>
				</EmptyState>
			{:else if isJobsLoaded(jobsState)}
				<DataTable data={jobsState.jobs} initialStatusFilter={statusFilter} />
			{/if}
		</Card.Content>
	</Card.Root>

	<SupportCard userEmail={user.email} userId={user.uid} />
</div>

<style>
	.submissions-page {
		display: flex;
		flex-direction: column;
		gap: var(--spacing-lg);
	}

	.page-header__title {
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.2;
		letter-spacing: -0.025em;
	}

	@media (min-width: 640px) {
		.page-header__title {
			font-size: 1.875rem;
		}
	}

	.page-header__description {
		margin-top: 0.5rem;
		color: hsl(var(--muted-foreground));
	}
</style>
