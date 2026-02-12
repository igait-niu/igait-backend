<script lang="ts">
	import { onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import {
		subscribeToQueues,
		subscribeToQueueConfigs,
		isQueuesLoading,
		isQueuesError,
		isQueuesLoaded,
		isQueueConfigLoaded,
		setQueueRequiresApproval,
		queueItemToJob,
		type QueuesState,
		type QueuesData,
		type QueueConfigState,
		type QueueItem,
		type FinalizeQueueItem
	} from '$lib/hooks';
	import { Badge } from '$lib/components/ui/badge';
	import { Switch } from '$lib/components/ui/switch';
	import { JobsDataTable } from '$lib/components/jobs';
	import { Inbox } from '@lucide/svelte';
	import AdminLoadingState from '../AdminLoadingState.svelte';
	import AdminErrorState from '../AdminErrorState.svelte';
	import type { Job } from '../../../../types/Job';

	// ── State ──────────────────────────────────────────────

	let queuesState: QueuesState = $state({ status: 'loading' });
	let configState: QueueConfigState = $state({ status: 'loading' });
	let activeStage: string = $state('stage_1');

	// ── Subscriptions ──────────────────────────────────────

	const unsubQueues = subscribeToQueues((state) => {
		queuesState = state;
	});

	const unsubConfigs = subscribeToQueueConfigs((state) => {
		configState = state;
	});

	onDestroy(() => {
		unsubQueues();
		unsubConfigs();
	});

	// ── Stage info ─────────────────────────────────────────

	const stageInfo = [
		{ key: 'stage_1', name: 'Stage 1', description: 'Media Conversion' },
		{ key: 'stage_2', name: 'Stage 2', description: 'Validity Check' },
		{ key: 'stage_3', name: 'Stage 3', description: 'Reframing' },
		{ key: 'stage_4', name: 'Stage 4', description: 'Pose Estimation' },
		{ key: 'stage_5', name: 'Stage 5', description: 'Cycle Detection' },
		{ key: 'stage_6', name: 'Stage 6', description: 'ML Prediction' },
		{ key: 'finalize', name: 'Stage 7', description: 'Finalize' }
	] as const;

	// ── Derived data ───────────────────────────────────────

	/** Get the count of items in a queue */
	function getQueueItemCount(key: string): number {
		if (!isQueuesLoaded(queuesState)) return 0;
		const queue = queuesState.queues[key as keyof QueuesData];
		return Object.keys(queue || {}).length;
	}

	/** Items for the active stage, preserving RTDB keys */
	const activeQueueEntries = $derived.by(() => {
		if (!isQueuesLoaded(queuesState)) return [];
		const queue = queuesState.queues[activeStage as keyof QueuesData];
		if (!queue) return [];
		return Object.entries(queue).map(([key, item]) => ({
			key,
			item: item as QueueItem | FinalizeQueueItem
		}));
	});

	/** Queue items converted to Job format for the data table */
	const jobsForTable = $derived(activeQueueEntries.map(({ item }) => queueItemToJob(item)));

	/** Total jobs across all queues */
	const totalJobs = $derived.by(() => {
		return stageInfo.reduce((total, stage) => total + getQueueItemCount(stage.key), 0);
	});

	/** Whether the active stage's queue requires manual approval */
	const activeRequiresApproval = $derived.by(() => {
		if (!isQueueConfigLoaded(configState)) return false;
		return configState.configs[activeStage]?.requires_approval ?? false;
	});

	/** Active stage display info */
	const activeStageInfo = $derived(stageInfo.find((s) => s.key === activeStage) ?? stageInfo[0]);

	// ── Handlers ───────────────────────────────────────────

	function handleSelectStage(stageKey: string) {
		activeStage = stageKey;
	}

	function handleSelectJob(job: Job & { id: string }) {
		goto(`/job/${encodeURIComponent(job.id)}`);
	}

	async function handleToggleApproval(value: boolean) {
		await setQueueRequiresApproval(activeStage, value);
	}
</script>

<svelte:head>
	<title>Queue Overview - Admin - iGait</title>
</svelte:head>

{#if isQueuesLoading(queuesState)}
	<AdminLoadingState message="Loading queues..." />
{:else if isQueuesError(queuesState)}
	<AdminErrorState message="Failed to load queues: {queuesState.error}" />
{:else if isQueuesLoaded(queuesState)}
	<div class="queue-overview">
		<!-- Header -->
		<header class="overview-header">
			<div class="header-left">
				<h2>Pipeline Status</h2>
				<Badge variant="secondary">{totalJobs} active</Badge>
			</div>
		</header>

		<!-- Stage Tabs -->
		<div class="stage-tabs">
			{#each stageInfo as stage}
				{@const count = getQueueItemCount(stage.key)}
				<button
					class="stage-tab"
					class:active={activeStage === stage.key}
					onclick={() => handleSelectStage(stage.key)}
				>
					<span class="tab-name">{stage.name}</span>
					<Badge variant={count > 0 ? 'default' : 'outline'} class="tab-badge">
						{count}
					</Badge>
				</button>
			{/each}
		</div>

		<!-- Active Stage Controls -->
		<div class="stage-controls">
			<span class="stage-description"><b>{activeStageInfo.description}</b></span>

			<label class="approval-toggle">
				<Switch checked={activeRequiresApproval} onCheckedChange={handleToggleApproval} />
				<span class="toggle-label">Require Manual Approval</span>
			</label>
		</div>

		<!-- Main Content -->
		<div class="content-area">
			{#if jobsForTable.length === 0}
				<div class="empty-state">
					<Inbox class="empty-icon" />
					<p class="empty-title">No jobs in queue</p>
					<p class="empty-description">
						{activeStageInfo.description} has no pending items right now.
					</p>
				</div>
			{:else}
				<JobsDataTable data={jobsForTable} uid="" showEmail={true} onRowClick={handleSelectJob} />
			{/if}
		</div>
	</div>
{/if}

<style>
	.queue-overview {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.overview-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.header-left {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.overview-header h2 {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}

	/* ── Stage Tabs ─────────────────────────────────────── */

	.stage-tabs {
		display: flex;
		gap: 0.125rem;
		overflow-x: auto;
		overflow-y: clip;
		border-bottom: 2px solid hsl(var(--border));
		padding-bottom: 0;
	}

	.stage-tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.625rem 0.875rem;
		border: none;
		background: none;
		cursor: pointer;
		font-size: 0.8125rem;
		font-weight: 500;
		color: hsl(var(--muted-foreground));
		border-bottom: 2px solid transparent;
		transition:
			color 0.15s ease,
			border-color 0.15s ease,
			background-color 0.15s ease;
		white-space: nowrap;
		margin-bottom: -2px;
		border-radius: var(--radius-sm) var(--radius-sm) 0 0;
	}

	.stage-tab:hover {
		color: hsl(var(--foreground));
		background-color: hsl(var(--muted) / 0.5);
	}

	.stage-tab.active {
		color: hsl(var(--foreground));
		font-weight: 600;
		border-bottom-color: hsl(var(--primary));
	}

	.tab-name {
		font-weight: inherit;
	}

	:global(.tab-badge) {
		font-size: 0.625rem;
		padding: 0.0625rem 0.375rem;
		height: auto;
		min-width: 1.25rem;
		text-align: center;
	}

	/* ── Stage Controls ─────────────────────────────────── */

	.stage-controls {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.625rem 0.875rem;
		background: hsl(var(--muted) / 0.35);
		border: 1px solid hsl(var(--border));
		border-radius: var(--radius-md);
	}

	.stage-description {
		font-size: 0.8125rem;
		color: hsl(var(--muted-foreground));
		font-weight: 500;
	}

	.approval-toggle {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		cursor: pointer;
	}

	.toggle-label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: hsl(var(--foreground));
		user-select: none;
	}

	/* ── Content Area ───────────────────────────────────── */

	.content-area {
		display: flex;
		gap: 1rem;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		text-align: center;
		padding: 3.5rem 2rem;
		color: hsl(var(--muted-foreground));
		background: hsl(var(--card));
		border: 1px dashed hsl(var(--border));
		border-radius: var(--radius-md);
	}

	:global(.empty-icon) {
		width: 2.5rem;
		height: 2.5rem;
		color: hsl(var(--muted-foreground) / 0.5);
		margin-bottom: 0.75rem;
	}

	.empty-title {
		font-size: 0.9375rem;
		font-weight: 600;
		color: hsl(var(--foreground));
		margin: 0 0 0.25rem;
	}

	.empty-description {
		font-size: 0.8125rem;
		margin: 0;
		max-width: 20rem;
		line-height: 1.4;
	}
</style>
