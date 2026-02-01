<script lang="ts">
	import { getUser } from '$lib/hooks';
	import { errorStore } from '$lib/stores';
	import { submitContribution, type ContributionRequest, type ProgressCallback } from '$lib/api';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import { Upload, Video, CheckCircle, Loader2 } from '@lucide/svelte';
	import { toast } from 'svelte-sonner';
	import { goto } from '$app/navigation';
	import { type Option, None, Some, AppError } from '$lib/result';

	const user = getUser();

	let name = $state('');
	let frontVideo: File | undefined = $state(undefined);
	let sideVideo: File | undefined = $state(undefined);
	let isSubmitting = $state(false);
	let progress = $state(0);
	let error: Option<AppError> = $state(None());

	function handleFileChange(e: Event, type: 'front' | 'side') {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		if (file) {
			if (type === 'front') {
				frontVideo = file;
			} else {
				sideVideo = file;
			}
		}
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		error = None();

		if (!frontVideo || !sideVideo) {
			error = Some(new AppError('Please select both front and side videos'));
			return;
		}

		isSubmitting = true;
		progress = 0;

		const request: ContributionRequest = {
			uid: user.uid,
			email: user.email,
			name: name.trim() || user.displayName,
			frontVideo,
			sideVideo
		};

		const onProgress: ProgressCallback = (p) => {
			progress = p;
		};

		const result = await submitContribution(request, onProgress);

		if (result.isErr()) {
			error = Some(result.error);
			isSubmitting = false;
			progress = 0;
		} else {
			toast.success(result.value);
			setTimeout(() => goto('/dashboard'), 1500);
		}
	}
</script>

<svelte:head>
	<title>New Submission - iGait</title>
</svelte:head>

<div class="space-y-8">
	<section>
		<h1 class="text-3xl font-bold tracking-tight">New Submission</h1>
		<p class="mt-2 text-muted-foreground">
			Upload your walking videos for gait analysis
		</p>
	</section>

	<Card.Root class="max-w-2xl">
		<Card.Header>
			<Card.Title>Upload Videos</Card.Title>
			<Card.Description>
				Please upload both a front view and side view video of walking.
				Videos should be clear and show the full body.
			</Card.Description>
		</Card.Header>
		<Card.Content>
			{#if error.isSome()}
				<Alert variant="destructive" class="mb-6">
					<AlertDescription>
						{error.value.displayMessage}
					</AlertDescription>
				</Alert>
			{/if}

			<form onsubmit={handleSubmit} class="space-y-6">
				<div class="space-y-2">
					<Label for="name">Subject Name (optional)</Label>
					<Input
						id="name"
						placeholder="Enter name or leave blank to use your display name"
						bind:value={name}
						disabled={isSubmitting}
					/>
				</div>

				<!-- Front Video Upload -->
				<div class="space-y-2">
					<Label for="frontVideo">Front View Video</Label>
					<div class="flex items-center gap-4">
						<label 
							class="flex h-32 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-muted-foreground/25 bg-muted/50 transition-colors hover:border-primary/50 hover:bg-muted"
						>
							<input
								id="frontVideo"
								type="file"
								accept="video/*"
								class="hidden"
								onchange={(e) => handleFileChange(e, 'front')}
								disabled={isSubmitting}
							/>
							{#if frontVideo}
								<CheckCircle class="mb-2 h-8 w-8 text-green-500" />
								<span class="text-sm font-medium">{frontVideo.name}</span>
								<span class="text-xs text-muted-foreground">
									{(frontVideo.size / 1024 / 1024).toFixed(1)} MB
								</span>
							{:else}
								<Video class="mb-2 h-8 w-8 text-muted-foreground" />
								<span class="text-sm text-muted-foreground">Click to upload front video</span>
							{/if}
						</label>
					</div>
				</div>

				<!-- Side Video Upload -->
				<div class="space-y-2">
					<Label for="sideVideo">Side View Video</Label>
					<div class="flex items-center gap-4">
						<label 
							class="flex h-32 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-muted-foreground/25 bg-muted/50 transition-colors hover:border-primary/50 hover:bg-muted"
						>
							<input
								id="sideVideo"
								type="file"
								accept="video/*"
								class="hidden"
								onchange={(e) => handleFileChange(e, 'side')}
								disabled={isSubmitting}
							/>
							{#if sideVideo}
								<CheckCircle class="mb-2 h-8 w-8 text-green-500" />
								<span class="text-sm font-medium">{sideVideo.name}</span>
								<span class="text-xs text-muted-foreground">
									{(sideVideo.size / 1024 / 1024).toFixed(1)} MB
								</span>
							{:else}
								<Video class="mb-2 h-8 w-8 text-muted-foreground" />
								<span class="text-sm text-muted-foreground">Click to upload side video</span>
							{/if}
						</label>
					</div>
				</div>

				<!-- Progress bar -->
				{#if isSubmitting && progress > 0}
					<div class="space-y-2">
						<div class="h-2 w-full overflow-hidden rounded-full bg-muted">
							<div 
								class="h-full bg-primary transition-all duration-300"
								style="width: {progress}%"
							></div>
						</div>
						<p class="text-center text-sm text-muted-foreground">
							Uploading... {progress}%
						</p>
					</div>
				{/if}

				<Button 
					type="submit" 
					class="w-full" 
					disabled={isSubmitting || !frontVideo || !sideVideo}
				>
					{#if isSubmitting}
						<Loader2 class="mr-2 h-4 w-4 animate-spin" />
						Uploading...
					{:else}
						<Upload class="mr-2 h-4 w-4" />
						Submit for Analysis
					{/if}
				</Button>
			</form>
		</Card.Content>
	</Card.Root>
</div>
