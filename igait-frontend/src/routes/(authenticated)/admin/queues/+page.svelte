<script lang="ts">
	import { onDestroy } from 'svelte';
	import { 
		subscribeToQueues, 
		subscribeToQueueConfigs,
		isQueuesLoading, 
		isQueuesError, 
		isQueuesLoaded,
		isQueueConfigLoaded,
		setQueueRequiresApproval,
		approveQueueItem,
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
	import AdminLoadingState from '../AdminLoadingState.svelte';
	import AdminErrorState from '../AdminErrorState.svelte';
	import QueueJobDetailPanel from './QueueJobDetailPanel.svelte';
	import type { Job } from '../../../../types/Job';

	// ── State ──────────────────────────────────────────────

	let queuesState: QueuesState = $state({ status: 'loading' });
	let configState: QueueConfigState = $state({ status: 'loading' });
	let activeStage: string = $state('stage_1');
	let selectedJobId: string | null = $state(null);

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
		{ key: 'stage_7', name: 'Stage 7', description: 'Finalize' },
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
	const jobsForTable = $derived(
		activeQueueEntries.map(({ item }) => queueItemToJob(item))
	);

	/** Total jobs across all queues */
	const totalJobs = $derived.by(() => {
		return stageInfo.reduce((total, stage) => total + getQueueItemCount(stage.key), 0);
	});

	/** Whether the active stage's queue requires manual approval */
	const activeRequiresApproval = $derived.by(() => {
		if (!isQueueConfigLoaded(configState)) return false;
		return configState.configs[activeStage]?.requires_approval ?? false;
	});

	/** The selected queue entry (item + rtdb key) */
	const selectedEntry = $derived.by(() => {
		if (!selectedJobId) return null;
		return activeQueueEntries.find(e => e.item.job_id === selectedJobId) ?? null;
	});

	/** Active stage display info */
	const activeStageInfo = $derived(
		stageInfo.find(s => s.key === activeStage) ?? stageInfo[0]
	);

	/** Whether this is the finalize stage (no approval toggle) */
	const isFinalize = $derived(activeStage === 'finalize');

	// ── Handlers ───────────────────────────────────────────

	function handleSelectStage(stageKey: string) {
		activeStage = stageKey;
		selectedJobId = null;
	}

	function handleSelectJob(job: Job & { id: string }) {
		selectedJobId = selectedJobId === job.id ? null : job.id;
	}

	async function handleToggleApproval(value: boolean) {
		await setQueueRequiresApproval(activeStage, value);
	}

	async function handleApproveJob() {
		if (!selectedEntry) return;
		await approveQueueItem(activeStage, selectedEntry.key, selectedEntry.item);
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
			<span class="stage-description">{activeStageInfo.description}</span>

			{#if !isFinalize}
				<label class="approval-toggle">
					<Switch
						checked={activeRequiresApproval}
						onCheckedChange={handleToggleApproval}
					/>
					<span class="toggle-label">Require Manual Approval</span>
				</label>
			{/if}
		</div>

		<!-- Main Content -->
		<div class="content-area" class:has-detail={selectedEntry !== null}>
			<!-- Jobs Table -->
			<div class="table-column">
				{#if jobsForTable.length === 0}
					<div class="empty-state">
						<p>No jobs in this queue</p>
					</div>
				{:else}
					<JobsDataTable 
						data={jobsForTable}
						uid=""
						showEmail={true}
						selectedId={selectedJobId}
						onRowClick={handleSelectJob}
						onViewDetails={handleSelectJob}
					/>
				{/if}
			</div>

			<!-- Detail Panel -->
			{#if selectedEntry}
				<div class="detail-column">
					<QueueJobDetailPanel
						item={selectedEntry.item}
						queueRequiresApproval={activeRequiresApproval}
						onApprove={handleApproveJob}
						onClose={() => selectedJobId = null}
					/>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.queue-overview {
		display: flex;
		flex-direction: column;
		gap: 1rem;
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
		gap: 0.25rem;
		overflow-x: auto;
		border-bottom: 1px solid hsl(var(--border));
		padding-bottom: 0;
	}

	.stage-tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.5rem 0.75rem;
		border: none;
		background: none;
		cursor: pointer;
		font-size: 0.8125rem;
		font-weight: 500;
		color: hsl(var(--muted-foreground));
		border-bottom: 2px solid transparent;
		transition: color 0.15s ease, border-color 0.15s ease;
		white-space: nowrap;
		margin-bottom: -1px;
	}

	.stage-tab:hover {
		color: hsl(var(--foreground));
	}

	.stage-tab.active {
		color: hsl(var(--foreground));
		border-bottom-color: hsl(var(--primary));
	}

	.tab-name {
		font-weight: inherit;
	}

	:global(.tab-badge) {
		font-size: 0.625rem;
		padding: 0.0625rem 0.375rem;
		height: auto;
	}

	/* ── Stage Controls ─────────────────────────────────── */

	.stage-controls {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
	}

	.stage-description {
		font-size: 0.8125rem;
		color: hsl(var(--muted-foreground));
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

	.table-column {
		flex: 1;
		min-width: 0;
	}

	.detail-column {
		flex-shrink: 0;
		width: 320px;
	}

	.content-area:not(.has-detail) .table-column {
		flex: 1;
	}

	.empty-state {
		text-align: center;
		padding: 3rem 2rem;
		color: hsl(var(--muted-foreground));
		font-size: 0.875rem;
		background: hsl(var(--card));
		border: 1px solid hsl(var(--border));
		border-radius: 0.5rem;
	}

	/* ── Responsive ─────────────────────────────────────── */

	@media (max-width: 900px) {
		.content-area {
			flex-direction: column;
		}

		.detail-column {
			width: 100%;
		}
	}
</style>
