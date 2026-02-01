<script lang="ts">
	import { onDestroy } from 'svelte';
	import { 
		subscribeToQueues, 
		isQueuesLoading, 
		isQueuesError, 
		isQueuesLoaded,
		type QueuesState,
		type QueueItem,
		type FinalizeQueueItem
	} from '$lib/hooks';
	import { Badge } from '$lib/components/ui/badge';
	import AdminLoadingState from '../AdminLoadingState.svelte';
	import AdminErrorState from '../AdminErrorState.svelte';
	import StageCard from './StageCard.svelte';

	let queuesState: QueuesState = $state({ status: 'loading' });

	const unsubscribe = subscribeToQueues((state) => {
		queuesState = state;
	});

	onDestroy(() => {
		unsubscribe();
	});

	const stageInfo = [
		{ key: 'stage_1', name: 'Stage 1', description: 'Media Conversion', color: 'bg-blue-500' },
		{ key: 'stage_2', name: 'Stage 2', description: 'Validity Check', color: 'bg-cyan-500' },
		{ key: 'stage_3', name: 'Stage 3', description: 'Reframing', color: 'bg-teal-500' },
		{ key: 'stage_4', name: 'Stage 4', description: 'Pose Estimation', color: 'bg-green-500' },
		{ key: 'stage_5', name: 'Stage 5', description: 'Cycle Detection', color: 'bg-lime-500' },
		{ key: 'stage_6', name: 'Stage 6', description: 'ML Prediction', color: 'bg-yellow-500' },
		{ key: 'finalize', name: 'Finalize', description: 'Stage 7', color: 'bg-amber-500' },
	] as const;

	function getQueueItems(queues: QueuesState, key: string): (QueueItem | FinalizeQueueItem)[] {
		if (!isQueuesLoaded(queues)) return [];
		const queue = queues.queues[key as keyof typeof queues.queues];
		return Object.values(queue || {});
	}

	const totalJobs = $derived.by(() => {
		if (!isQueuesLoaded(queuesState)) return 0;
		return stageInfo.reduce((total, stage) => {
			return total + getQueueItems(queuesState, stage.key).length;
		}, 0);
	});
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
		<header class="overview-header">
			<div class="header-left">
				<h2>Pipeline Status</h2>
				<Badge variant="secondary">{totalJobs} active</Badge>
			</div>
		</header>

		<div class="pipeline-grid">
			{#each stageInfo as stage}
				{@const items = getQueueItems(queuesState, stage.key)}
				<StageCard 
					name={stage.name}
					description={stage.description}
					colorClass={stage.color}
					{items}
				/>
			{/each}
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

	.pipeline-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
		gap: 0.75rem;
	}

	@media (min-width: 1200px) {
		.pipeline-grid {
			grid-template-columns: repeat(4, 1fr);
		}
	}

	@media (min-width: 900px) and (max-width: 1199px) {
		.pipeline-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}
</style>
