<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog';
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
		AlertTriangle
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

	function getStatusVariant(status: JobStatus): 'default' | 'secondary' | 'destructive' | 'outline' {
		switch (status.code) {
			case 'Complete':
				return status.asd ? 'destructive' : 'default';
			case 'Error': return 'destructive';
			case 'Processing': return 'secondary';
			case 'Submitted':
			default: return 'outline';
		}
	}

	function getStatusIcon(status: JobStatus) {
		switch (status.code) {
			case 'Complete': return CheckCircle2;
			case 'Error': return XCircle;
			case 'Processing': return Clock;
			case 'Submitted':
			default: return Clock;
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
</script>

<Dialog.Root open={true} onOpenChange={onClose}>
	<Dialog.Content class="sm:max-w-[600px] max-h-[80vh] overflow-y-auto">
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
					<h3 class="text-sm font-medium flex items-center gap-2">
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
				<h3 class="text-sm font-medium flex items-center gap-2">
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
				<h3 class="text-sm font-medium flex items-center gap-2">
					<UserIcon class="h-4 w-4" />
					Patient Information
				</h3>
				<div class="grid grid-cols-2 gap-3 text-sm">
					<div>
						<span class="text-muted-foreground block mb-1">Age</span>
						<span class="font-medium">{job.age} years</span>
					</div>
					<div>
						<span class="text-muted-foreground block mb-1">Sex</span>
						<span class="font-medium">{job.sex}</span>
					</div>
					<div>
						<span class="text-muted-foreground block mb-1">Height</span>
						<span class="font-medium">{job.height}</span>
					</div>
					<div>
						<span class="text-muted-foreground block mb-1">Weight</span>
						<span class="font-medium">{job.weight} lbs</span>
					</div>
					<div class="col-span-2">
						<span class="text-muted-foreground block mb-1">Ethnicity</span>
						<span class="font-medium">{job.ethnicity}</span>
					</div>
				</div>
			</div>

			{#if isComplete && completeResult}
				<Separator />
				
				<!-- Results Section -->
				<div class="space-y-3">
					<h3 class="text-sm font-medium flex items-center gap-2">
						<CheckCircle2 class="h-4 w-4" />
						Analysis Results
					</h3>
					<div class="grid gap-3 text-sm">
						<div class="flex justify-between items-center p-3 bg-muted rounded-lg">
							<span class="text-muted-foreground">ASD Detection:</span>
							<Badge variant={completeResult.asd ? 'destructive' : 'default'}>
								{completeResult.asd ? 'ASD Indicators Detected' : 'No ASD Indicators'}
							</Badge>
						</div>
						<div class="flex justify-between items-center p-3 bg-muted rounded-lg">
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
					<h3 class="text-sm font-medium flex items-center gap-2 text-destructive">
						<AlertTriangle class="h-4 w-4" />
						Error Details
					</h3>
					<div class="p-3 bg-destructive/10 rounded-lg border border-destructive/20">
						<pre class="text-xs text-destructive whitespace-pre-wrap break-words max-h-32 overflow-y-auto">{errorLogs}</pre>
					</div>
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
