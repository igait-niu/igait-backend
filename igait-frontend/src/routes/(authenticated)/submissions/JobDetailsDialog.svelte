<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Accordion from '$lib/components/ui/accordion';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Button } from '$lib/components/ui/button';
	import * as Progress from '$lib/components/ui/progress';
	import {
		FileVideo,
		CheckCircle2,
		Clock,
		XCircle,
		User as UserIcon,
		Calendar,
		Download,
		AlertTriangle,
		ScrollText
	} from '@lucide/svelte';
	import type { Job } from '../../../types/Job';
	import type { JobStatus } from '../../../types/JobStatus';

	type Props = {
		job: Job;
		onClose: () => void;
	};

	let { job, onClose }: Props = $props();

	function formatDate(timestamp: number): string {
		const date = new Date(timestamp * 1000);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function getStatusVariant(
		status: JobStatus
	): 'default' | 'secondary' | 'destructive' | 'outline' {
		switch (status.code) {
			case 'Complete':
				return status.asd ? 'destructive' : 'default';
			case 'Error':
				return 'destructive';
			case 'Processing':
				return 'secondary';
			case 'Submitted':
			default:
				return 'outline';
		}
	}

	function getStatusIcon(status: JobStatus) {
		switch (status.code) {
			case 'Complete':
				return CheckCircle2;
			case 'Error':
				return XCircle;
			case 'Processing':
				return Clock;
			case 'Submitted':
			default:
				return Clock;
		}
	}

	const statusVariant = $derived(getStatusVariant(job.status));
	const StatusIcon = $derived(getStatusIcon(job.status));
	const formattedDate = $derived(formatDate(job.timestamp));
	const isComplete = $derived(job.status.code === 'Complete');
	const isProcessing = $derived(job.status.code === 'Processing');
	const isError = $derived(job.status.code === 'Error');

	// Processing progress
	const processingProgress = $derived.by(() => {
		if (job.status.code === 'Processing') {
			return (job.status.stage / job.status.num_stages) * 100;
		}
		return 0;
	});

	// Complete results
	const completeResult = $derived.by(() => {
		if (job.status.code === 'Complete') {
			return job.status;
		}
		return null;
	});

	// Error logs
	const errorLogs = $derived.by(() => {
		if (job.status.code === 'Error') {
			return job.status.logs;
		}
		return null;
	});

	// Stage logs - sorted entries from stage_logs HashMap
	const STAGE_NAMES: Record<string, string> = {
		stage_1: 'Media Conversion',
		stage_2: 'Validity Check',
		stage_3: 'Reframing',
		stage_4: 'Pose Estimation',
		stage_5: 'Cycle Detection',
		stage_6: 'ML Prediction',
		stage_7: 'Finalize'
	};

	const stageLogs = $derived.by(() => {
		if (!job.stage_logs) return [];
		return Object.entries(job.stage_logs)
			.sort(([a], [b]) => a.localeCompare(b))
			.map(([key, value]) => ({
				key,
				label: STAGE_NAMES[key] ?? key,
				stageNum: key.replace('stage_', ''),
				logs: value
			}));
	});
	const hasStageLogs = $derived(stageLogs.length > 0);
</script>

<Dialog.Root open={true} onOpenChange={onClose}>
	<Dialog.Content class="max-h-[80vh] overflow-y-auto sm:max-w-[600px]">
		<Dialog.Header>
			<Dialog.Title class="flex items-center gap-2">
				<FileVideo class="h-5 w-5" />
				Submission Details
			</Dialog.Title>
			<Dialog.Description>
				Complete information about this gait analysis submission
			</Dialog.Description>
		</Dialog.Header>

		<div class="space-y-6 py-4">
			<!-- Status Section -->
			<div class="space-y-3">
				{#if StatusIcon}
					{@const Icon = StatusIcon}
					<h3 class="flex items-center gap-2 text-sm font-medium">
						<Icon class="h-4 w-4" />
						Status
					</h3>
				{/if}
				<div class="flex items-center gap-2">
					<Badge variant={statusVariant} class="text-sm">
						{job.status.value}
					</Badge>
				</div>

				<!-- Processing Progress Bar -->
				{#if isProcessing && job.status.code === 'Processing'}
					<div class="space-y-2">
						<Progress.Root value={processingProgress} class="w-full" />
						<p class="text-xs text-muted-foreground">
							Stage {job.status.stage} of {job.status.num_stages}
						</p>
					</div>
				{/if}
			</div>

			<Separator />

			<!-- Submission Info -->
			<div class="space-y-3">
				<h3 class="flex items-center gap-2 text-sm font-medium">
					<Calendar class="h-4 w-4" />
					Submission Information
				</h3>
				<div class="grid gap-2 text-sm">
					<div class="flex justify-between">
						<span class="text-muted-foreground">Submitted:</span>
						<span class="font-medium">{formattedDate}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-muted-foreground">Email:</span>
						<span class="font-medium">{job.email}</span>
					</div>
				</div>
			</div>

			<Separator />

			<!-- Patient Information -->
			<div class="space-y-3">
				<h3 class="flex items-center gap-2 text-sm font-medium">
					<UserIcon class="h-4 w-4" />
					Patient Information
				</h3>
				<div class="grid grid-cols-2 gap-3 text-sm">
					<div>
						<span class="mb-1 block text-muted-foreground">Age</span>
						<span class="font-medium">{job.age} years</span>
					</div>
					<div>
						<span class="mb-1 block text-muted-foreground">Sex</span>
						<span class="font-medium">{job.sex}</span>
					</div>
					<div>
						<span class="mb-1 block text-muted-foreground">Height</span>
						<span class="font-medium">{job.height}</span>
					</div>
					<div>
						<span class="mb-1 block text-muted-foreground">Weight</span>
						<span class="font-medium">{job.weight} lbs</span>
					</div>
					<div class="col-span-2">
						<span class="mb-1 block text-muted-foreground">Ethnicity</span>
						<span class="font-medium">{job.ethnicity}</span>
					</div>
				</div>
			</div>

			{#if isComplete && completeResult}
				<Separator />

				<!-- Results Section -->
				<div class="space-y-3">
					<h3 class="flex items-center gap-2 text-sm font-medium">
						<CheckCircle2 class="h-4 w-4" />
						Analysis Results
					</h3>
					<div class="grid gap-3 text-sm">
						<div class="flex items-center justify-between rounded-lg bg-muted p-3">
							<span class="text-muted-foreground">ASD Detection:</span>
							<Badge variant={completeResult.asd ? 'destructive' : 'default'}>
								{completeResult.asd ? 'ASD Indicators Detected' : 'No ASD Indicators'}
							</Badge>
						</div>
						<div class="flex items-center justify-between rounded-lg bg-muted p-3">
							<span class="text-muted-foreground">Confidence:</span>
							<span class="font-medium">
								{completeResult.asd
									? (completeResult.prediction * 100).toFixed(1)
									: ((1 - completeResult.prediction) * 100).toFixed(1)}%
							</span>
						</div>
					</div>
					<Button class="w-full" size="sm">
						<Download class="mr-2 h-4 w-4" />
						Download Full Report
					</Button>
				</div>
			{/if}

			{#if isError && errorLogs}
				<Separator />

				<!-- Error Section -->
				<div class="space-y-3">
					<h3 class="flex items-center gap-2 text-sm font-medium text-destructive">
						<AlertTriangle class="h-4 w-4" />
						Error Details
					</h3>
					<div class="rounded-lg border border-destructive/20 bg-destructive/10 p-3">
						<pre
							class="max-h-32 overflow-y-auto text-xs break-words whitespace-pre-wrap text-destructive">{errorLogs}</pre>
					</div>
				</div>
			{/if}

			{#if hasStageLogs}
				<Separator />

				<!-- Stage Logs Section -->
				<div class="space-y-3">
					<h3 class="flex items-center gap-2 text-sm font-medium">
						<ScrollText class="h-4 w-4" />
						Stage Logs
					</h3>
					<Accordion.Root type="multiple">
						{#each stageLogs as { key, label, stageNum, logs } (key)}
							<Accordion.Item value={key}>
								<Accordion.Trigger class="text-sm">
									<span class="flex items-center gap-2">
										<Badge variant="outline" class="px-1.5 font-mono text-xs">{stageNum}</Badge>
										{label}
									</span>
								</Accordion.Trigger>
								<Accordion.Content>
									<div class="rounded-lg bg-muted p-3">
										<pre
											class="max-h-48 overflow-y-auto font-mono text-xs break-words whitespace-pre-wrap">{logs}</pre>
									</div>
								</Accordion.Content>
							</Accordion.Item>
						{/each}
					</Accordion.Root>
				</div>
			{/if}
		</div>

		<Dialog.Footer>
			<Button variant="outline" onclick={onClose}>Close</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<style>
	:global(.max-h-\[80vh\]) {
		max-height: 80vh;
	}
</style>
