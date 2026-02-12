<script lang="ts">
	import { onDestroy } from 'svelte';
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { subscribeToJob, type SingleJobState } from '$lib/hooks';
	import { rerunJob } from '$lib/api';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import * as Dialog from '$lib/components/ui/dialog';
	import {
		ArrowLeft,
		RotateCcw,
		AlertTriangle,
		ScrollText,
		FileInput,
		FileOutput
	} from '@lucide/svelte';
	import AdminLoadingState from '../../AdminLoadingState.svelte';
	import AdminErrorState from '../../AdminErrorState.svelte';
	import type { Job } from '../../../../../types/Job';

	// ── Route param ────────────────────────────────────────
	const jobId = $derived($page.params.id);

	// ── State ──────────────────────────────────────────────
	let jobState: SingleJobState = $state({ status: 'loading' });
	let activeStage: string = $state('stage_1');
	let activeSubTab: 'input' | 'output' | 'logs' = $state('logs');
	let showRerunDialog = $state(false);
	let rerunLoading = $state(false);
	let rerunError: string | null = $state(null);
	let rerunSuccess: string | null = $state(null);

	// ── Subscription ───────────────────────────────────────
	let unsubscribe: (() => void) | undefined;

	$effect(() => {
		// Clean up previous subscription
		unsubscribe?.();

		if (jobId) {
			unsubscribe = subscribeToJob(jobId, (state) => {
				jobState = state;
			});
		}
	});

	onDestroy(() => {
		unsubscribe?.();
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

	// ── Derived ────────────────────────────────────────────
	const job = $derived(jobState.status === 'loaded' ? jobState.job : null);

	const activeStageInfo = $derived(
		stageInfo.find(s => s.key === activeStage) ?? stageInfo[0]
	);

	const activeStageNumber = $derived(
		parseInt(activeStage.replace('stage_', ''), 10)
	);

	/** Logs for the currently selected stage */
	const currentStageLogs = $derived.by(() => {
		if (!job?.stage_logs) return null;
		return job.stage_logs[activeStage] ?? null;
	});

	/** Parse the job index from the composite ID */
	const jobIndex = $derived.by(() => {
		const lastUnderscore = jobId.lastIndexOf('_');
		if (lastUnderscore === -1) return 0;
		return parseInt(jobId.slice(lastUnderscore + 1), 10);
	});

	/** Formatted job ID for display */
	function formatJobId(id: string): string {
		const lastUnderscore = id.lastIndexOf('_');
		if (lastUnderscore === -1) return id;
		const uid = id.slice(0, lastUnderscore);
		const index = id.slice(lastUnderscore + 1);
		return `${uid.slice(0, 8)}…#${index}`;
	}

	// ── Handlers ───────────────────────────────────────────
	function handleBack() {
		history.back();
	}

	function handleStageClick(stageKey: string) {
		activeStage = stageKey;
	}

	function handleSubTabClick(tab: 'input' | 'output' | 'logs') {
		activeSubTab = tab;
	}

	function handleRerunClick() {
		rerunError = null;
		rerunSuccess = null;
		showRerunDialog = true;
	}

	async function handleRerunConfirm() {
		rerunLoading = true;
		rerunError = null;

		const result = await rerunJob(jobIndex, activeStageNumber);

		if (result.isOk()) {
			rerunSuccess = result.value.message;
			showRerunDialog = false;
		} else {
			rerunError = result.error.rootCause;
		}

		rerunLoading = false;
	}
</script>

<svelte:head>
	<title>Job {formatJobId(jobId)} - Admin - iGait</title>
</svelte:head>

{#if jobState.status === 'loading'}
	<AdminLoadingState message="Loading job details..." />
{:else if jobState.status === 'error'}
	<AdminErrorState message="Failed to load job: {jobState.error}" />
{:else if job}
	<div class="job-detail-page">
		<!-- Header -->
		<header class="detail-header">
			<Button variant="ghost" size="sm" class="back-btn" onclick={handleBack}>
				<ArrowLeft class="h-4 w-4" />
				Back
			</Button>
			<div class="header-info">
				<h2 class="header-title">
					Job <span class="mono">{formatJobId(jobId)}</span>
				</h2>
				<Badge variant="secondary">{job.email}</Badge>
			</div>
		</header>

		<!-- Stage Tabs -->
		<div class="stage-tabs">
			{#each stageInfo as stage}
				<button
					class="stage-tab"
					class:active={activeStage === stage.key}
					onclick={() => handleStageClick(stage.key)}
				>
					<span class="tab-name">{stage.name}</span>
					<span class="tab-desc">{stage.description}</span>
				</button>
			{/each}
		</div>

		<!-- Sub-tabs + Re-Run row -->
		<div class="sub-tab-row">
			<div class="sub-tabs">
				<button
					class="sub-tab"
					class:active={activeSubTab === 'input'}
					onclick={() => handleSubTabClick('input')}
				>
					<FileInput class="sub-tab-icon" />
					Input
				</button>
				<button
					class="sub-tab"
					class:active={activeSubTab === 'output'}
					onclick={() => handleSubTabClick('output')}
				>
					<FileOutput class="sub-tab-icon" />
					Output
				</button>
				<button
					class="sub-tab"
					class:active={activeSubTab === 'logs'}
					onclick={() => handleSubTabClick('logs')}
				>
					<ScrollText class="sub-tab-icon" />
					Logs
				</button>
			</div>

			<Button variant="destructive" size="sm" onclick={handleRerunClick}>
				<RotateCcw class="h-4 w-4 mr-1" />
				Re-Run
			</Button>
		</div>

		<!-- Success banner -->
		{#if rerunSuccess}
			<div class="success-banner">
				{rerunSuccess}
			</div>
		{/if}

		<!-- Tab Content -->
		<div class="tab-content">
			{#if activeSubTab === 'input'}
				<div class="placeholder-content">
					<p class="placeholder-label">Input for {activeStageInfo.name}: {activeStageInfo.description}</p>
					<p class="placeholder-face">:3</p>
				</div>
			{:else if activeSubTab === 'output'}
				<div class="placeholder-content">
					<p class="placeholder-label">Output for {activeStageInfo.name}: {activeStageInfo.description}</p>
					<p class="placeholder-face">:3</p>
				</div>
			{:else if activeSubTab === 'logs'}
				<div class="logs-content">
					{#if currentStageLogs}
						<pre class="log-output">{currentStageLogs}</pre>
					{:else}
						<div class="empty-logs">
							<ScrollText class="empty-icon" />
							<p>No logs available for {activeStageInfo.name}.</p>
						</div>
					{/if}
				</div>
			{/if}
		</div>
	</div>

	<!-- Re-Run Warning Dialog -->
	<Dialog.Root bind:open={showRerunDialog}>
		<Dialog.Content class="sm:max-w-[480px]">
			<Dialog.Header>
				<Dialog.Title class="flex items-center gap-2 text-destructive">
					<AlertTriangle class="h-5 w-5" />
					Confirm Re-Run
				</Dialog.Title>
				<Dialog.Description>
					This action cannot be undone.
				</Dialog.Description>
			</Dialog.Header>

			<div class="rerun-warning-body">
				<p>
					You are about to re-run <strong>{formatJobId(jobId)}</strong> starting
					from <strong>{activeStageInfo.name} ({activeStageInfo.description})</strong>.
				</p>
				<div class="warning-callout">
					<AlertTriangle class="callout-icon" />
					<span>
						This will <strong>clear all outputs</strong> from Stage {activeStageNumber}
						onward (through Stage 7). The job will be re-queued for processing.
					</span>
				</div>

				{#if rerunError}
					<div class="rerun-error">
						{rerunError}
					</div>
				{/if}
			</div>

			<Dialog.Footer>
				<Button variant="outline" onclick={() => showRerunDialog = false} disabled={rerunLoading}>
					Cancel
				</Button>
				<Button variant="destructive" onclick={handleRerunConfirm} disabled={rerunLoading}>
					{#if rerunLoading}
						Re-Running…
					{:else}
						<RotateCcw class="h-4 w-4 mr-1" />
						Re-Run from Stage {activeStageNumber}
					{/if}
				</Button>
			</Dialog.Footer>
		</Dialog.Content>
	</Dialog.Root>
{/if}

<style>
	.job-detail-page {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	/* ── Header ─────────────────────────────────────────── */

	.detail-header {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	:global(.back-btn) {
		align-self: flex-start;
		margin-left: -0.5rem;
	}

	.header-info {
		display: flex;
		align-items: center;
		gap: 0.75rem;
	}

	.header-title {
		font-size: 1.125rem;
		font-weight: 600;
		margin: 0;
	}

	.mono {
		font-family: ui-monospace, monospace;
		font-size: 0.9375rem;
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
		flex-direction: column;
		align-items: center;
		gap: 0.0625rem;
		padding: 0.625rem 0.875rem;
		border: none;
		background: none;
		cursor: pointer;
		color: hsl(var(--muted-foreground));
		border-bottom: 2px solid transparent;
		transition: color 0.15s ease, border-color 0.15s ease, background-color 0.15s ease;
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
		border-bottom-color: hsl(var(--primary));
	}

	.tab-name {
		font-size: 0.8125rem;
		font-weight: 600;
	}

	.tab-desc {
		font-size: 0.6875rem;
		font-weight: 400;
		opacity: 0.7;
	}

	/* ── Sub-tabs ───────────────────────────────────────── */

	.sub-tab-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		padding: 0.5rem 0.75rem;
		background: hsl(var(--muted) / 0.35);
		border: 1px solid hsl(var(--border));
		border-radius: var(--radius-md);
	}

	.sub-tabs {
		display: flex;
		gap: 0.25rem;
	}

	.sub-tab {
		display: flex;
		align-items: center;
		gap: 0.375rem;
		padding: 0.375rem 0.75rem;
		border: 1px solid transparent;
		background: none;
		border-radius: var(--radius-sm);
		cursor: pointer;
		font-size: 0.8125rem;
		font-weight: 500;
		color: hsl(var(--muted-foreground));
		transition: all 0.15s ease;
	}

	.sub-tab:hover {
		color: hsl(var(--foreground));
		background-color: hsl(var(--background));
	}

	.sub-tab.active {
		color: hsl(var(--foreground));
		background-color: hsl(var(--background));
		border-color: hsl(var(--border));
		font-weight: 600;
	}

	:global(.sub-tab-icon) {
		width: 0.875rem;
		height: 0.875rem;
	}

	/* ── Tab Content ────────────────────────────────────── */

	.tab-content {
		border: 1px solid hsl(var(--border));
		border-radius: var(--radius-md);
		background: hsl(var(--card));
		min-height: 300px;
	}

	.placeholder-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 2rem;
		text-align: center;
		color: hsl(var(--muted-foreground));
		gap: 0.5rem;
	}

	.placeholder-label {
		font-size: 0.875rem;
		margin: 0;
	}

	.placeholder-face {
		font-size: 2rem;
		margin: 0;
		opacity: 0.6;
	}

	.logs-content {
		padding: 1rem;
	}

	.log-output {
		font-family: ui-monospace, monospace;
		font-size: 0.75rem;
		line-height: 1.6;
		white-space: pre-wrap;
		word-break: break-word;
		background: hsl(var(--muted) / 0.4);
		padding: 1rem;
		border-radius: var(--radius-sm);
		max-height: 500px;
		overflow-y: auto;
		margin: 0;
	}

	.empty-logs {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 3rem 2rem;
		text-align: center;
		color: hsl(var(--muted-foreground));
		gap: 0.5rem;
	}

	:global(.empty-icon) {
		width: 2rem;
		height: 2rem;
		opacity: 0.4;
	}

	.empty-logs p {
		font-size: 0.875rem;
		margin: 0;
	}

	/* ── Re-Run Dialog ──────────────────────────────────── */

	.rerun-warning-body {
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
		padding: 0.5rem 0;
		font-size: 0.875rem;
	}

	.rerun-warning-body p {
		margin: 0;
		line-height: 1.5;
	}

	.warning-callout {
		display: flex;
		gap: 0.625rem;
		padding: 0.75rem;
		background: hsl(var(--destructive) / 0.08);
		border: 1px solid hsl(var(--destructive) / 0.2);
		border-radius: var(--radius-sm);
		font-size: 0.8125rem;
		line-height: 1.5;
		color: hsl(var(--destructive));
		align-items: flex-start;
	}

	:global(.callout-icon) {
		width: 1rem;
		height: 1rem;
		flex-shrink: 0;
		margin-top: 0.125rem;
	}

	.rerun-error {
		padding: 0.5rem 0.75rem;
		background: hsl(var(--destructive) / 0.1);
		border: 1px solid hsl(var(--destructive) / 0.3);
		border-radius: var(--radius-sm);
		font-size: 0.8125rem;
		color: hsl(var(--destructive));
	}

	.success-banner {
		padding: 0.625rem 0.875rem;
		background: hsl(142 76% 36% / 0.1);
		border: 1px solid hsl(142 76% 36% / 0.25);
		border-radius: var(--radius-md);
		font-size: 0.8125rem;
		color: hsl(142 76% 36%);
		font-weight: 500;
	}

	/* ── Responsive ─────────────────────────────────────── */

	@media (max-width: 768px) {
		.sub-tab-row {
			flex-direction: column;
			align-items: stretch;
		}

		.sub-tabs {
			justify-content: center;
		}
	}
</style>
