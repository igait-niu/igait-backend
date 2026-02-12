<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { Separator } from '$lib/components/ui/separator';
	import { User as UserIcon, Clock, CheckCircle2, ShieldCheck, X } from '@lucide/svelte';
	import type { QueueItem, FinalizeQueueItem } from '$lib/hooks';

	interface Props {
		item: QueueItem | FinalizeQueueItem;
		queueRequiresApproval: boolean;
		onApprove: () => void;
		onClose: () => void;
	}

	let { item, queueRequiresApproval, onApprove, onClose }: Props = $props();

	function getRelativeTime(timestamp: number): string {
		const seconds = Math.floor((Date.now() - timestamp) / 1000);
		if (seconds < 60) return `${seconds}s ago`;
		const minutes = Math.floor(seconds / 60);
		if (minutes < 60) return `${minutes}m ago`;
		const hours = Math.floor(minutes / 60);
		if (hours < 24) return `${hours}h ago`;
		const days = Math.floor(hours / 24);
		return `${days}d ago`;
	}

	function formatJobId(jobId: string): string {
		const parts = jobId.split('_');
		if (parts.length >= 2) {
			const index = parts[parts.length - 1];
			const uid = parts.slice(0, -1).join('_');
			return `${uid.slice(0, 8)}…#${index}`;
		}
		return jobId;
	}

	/**
	 * A job is effectively approved if:
	 * - `approved` is explicitly true, OR
	 * - Neither the job-level nor the queue-level flag requires approval
	 */
	const isEffectivelyApproved = $derived(
		item.approved || (!item.requires_approval && !queueRequiresApproval)
	);

	const needsManualReview = $derived(
		!item.approved && (item.requires_approval || queueRequiresApproval)
	);
</script>

<div class="detail-panel">
	<div class="panel-header">
		<h3 class="panel-title">Job Details</h3>
		<Button variant="ghost" size="sm" class="close-btn" onclick={onClose}>
			<X class="h-4 w-4" />
		</Button>
	</div>

	<div class="panel-body">
		<!-- Job Identity -->
		<div class="detail-section">
			<div class="detail-row">
				<span class="detail-label">Job ID</span>
				<span class="detail-value mono" title={item.job_id}>{formatJobId(item.job_id)}</span>
			</div>
			<div class="detail-row">
				<span class="detail-label">User ID</span>
				<span class="detail-value mono" title={item.user_id}>{item.user_id.slice(0, 12)}…</span>
			</div>
			{#if item.metadata?.email}
				<div class="detail-row">
					<span class="detail-label">Email</span>
					<span class="detail-value">{item.metadata.email}</span>
				</div>
			{/if}
			<div class="detail-row">
				<span class="detail-label">Enqueued</span>
				<span class="detail-value">
					<Clock class="inline-icon" />
					{getRelativeTime(item.enqueued_at)}
				</span>
			</div>
			<div class="detail-row">
				<span class="detail-label">Status</span>
				<Badge variant={item.claimed_by ? 'secondary' : 'outline'}>
					{item.claimed_by ? 'Claimed' : 'Queued'}
				</Badge>
			</div>
		</div>

		<Separator />

		<!-- Patient Information -->
		<div class="detail-section">
			<h4 class="section-title">
				<UserIcon class="section-icon" />
				Patient Information
			</h4>
			<div class="detail-grid">
				<div class="detail-cell">
					<span class="detail-label">Age</span>
					<span class="detail-value">{item.metadata?.age ?? '—'}</span>
				</div>
				<div class="detail-cell">
					<span class="detail-label">Sex</span>
					<span class="detail-value">{item.metadata?.sex ?? '—'}</span>
				</div>
				<div class="detail-cell">
					<span class="detail-label">Height</span>
					<span class="detail-value">{item.metadata?.height ?? '—'}</span>
				</div>
				<div class="detail-cell">
					<span class="detail-label">Weight</span>
					<span class="detail-value"
						>{item.metadata?.weight ? `${item.metadata.weight} lbs` : '—'}</span
					>
				</div>
				<div class="detail-cell full-width">
					<span class="detail-label">Ethnicity</span>
					<span class="detail-value">{item.metadata?.ethnicity ?? '—'}</span>
				</div>
			</div>
		</div>

		<Separator />

		<!-- Approval Section -->
		<div class="detail-section">
			<h4 class="section-title">
				<ShieldCheck class="section-icon" />
				Approval
			</h4>

			{#if isEffectivelyApproved}
				<Button variant="outline" size="sm" class="approve-btn" disabled>
					<CheckCircle2 class="btn-icon" />
					{item.approved ? 'Approved' : 'Auto-approved'}
				</Button>
			{:else if needsManualReview}
				<Button variant="default" size="sm" class="approve-btn" onclick={onApprove}>
					<ShieldCheck class="btn-icon" />
					Approve
				</Button>
			{/if}

			{#if item.requires_approval}
				<p class="approval-note">This job was submitted with manual approval requested.</p>
			{/if}
			{#if queueRequiresApproval && !item.requires_approval}
				<p class="approval-note">This queue requires manual approval for all jobs.</p>
			{/if}
		</div>
	</div>
</div>

<style>
	.detail-panel {
		background: hsl(var(--card));
		border: 1px solid hsl(var(--border));
		border-radius: 0.5rem;
		overflow: hidden;
		min-width: 280px;
	}

	.panel-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid hsl(var(--border));
	}

	.panel-title {
		font-size: 0.875rem;
		font-weight: 600;
		margin: 0;
	}

	:global(.close-btn) {
		width: 1.75rem !important;
		height: 1.75rem !important;
		padding: 0 !important;
	}

	.panel-body {
		padding: 0.75rem 1rem;
		display: flex;
		flex-direction: column;
		gap: 0.75rem;
	}

	.detail-section {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.section-title {
		font-size: 0.8125rem;
		font-weight: 600;
		margin: 0;
		display: flex;
		align-items: center;
		gap: 0.375rem;
	}

	:global(.section-icon) {
		width: 0.875rem;
		height: 0.875rem;
	}

	.detail-row {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 0.5rem;
	}

	.detail-label {
		font-size: 0.75rem;
		color: hsl(var(--muted-foreground));
		flex-shrink: 0;
	}

	.detail-value {
		font-size: 0.8125rem;
		font-weight: 500;
		text-align: right;
		display: flex;
		align-items: center;
		gap: 0.25rem;
	}

	.mono {
		font-family: ui-monospace, monospace;
		font-size: 0.75rem;
	}

	:global(.inline-icon) {
		width: 0.75rem;
		height: 0.75rem;
		color: hsl(var(--muted-foreground));
	}

	.detail-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 0.5rem;
	}

	.detail-cell {
		display: flex;
		flex-direction: column;
		gap: 0.125rem;
	}

	.full-width {
		grid-column: 1 / -1;
	}

	:global(.approve-btn) {
		width: 100%;
	}

	:global(.btn-icon) {
		width: 0.875rem;
		height: 0.875rem;
		margin-right: 0.375rem;
	}

	.approval-note {
		font-size: 0.6875rem;
		color: hsl(var(--muted-foreground));
		margin: 0;
		font-style: italic;
	}
</style>
