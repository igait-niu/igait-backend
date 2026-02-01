<script lang="ts">
	import { getUser } from '$lib/hooks';
	import { errorStore } from '$lib/stores';
	import { submitContribution, type ContributionRequest, type ProgressCallback } from '$lib/api';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Input } from '$lib/components/ui/input';
	import { Label } from '$lib/components/ui/label';
	import { Alert, AlertDescription } from '$lib/components/ui/alert';
	import { Loader2, Upload } from '@lucide/svelte';
	import { toast } from 'svelte-sonner';
	import { goto } from '$app/navigation';
	import { type Option, None, Some, AppError } from '$lib/result';
	import VideoUploadArea from './VideoUploadArea.svelte';

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

<div class="submit-page stack-lg">
	<section>
		<h1 class="page-title">New Submission</h1>
		<p class="page-description">
			Upload your walking videos for gait analysis
		</p>
	</section>

	<Card.Root class="submit-card">
		<Card.Header>
			<Card.Title>Upload Videos</Card.Title>
			<Card.Description>
				Please upload both a front view and side view video of walking.
				Videos should be clear and show the full body.
			</Card.Description>
		</Card.Header>
		<Card.Content>
			{#if error.isSome()}
				<Alert variant="destructive" class="form-error">
					<AlertDescription>
						{error.value.displayMessage}
					</AlertDescription>
				</Alert>
			{/if}

			<form onsubmit={handleSubmit} class="submit-form">
				<div class="form-group">
					<Label for="name">Subject Name (optional)</Label>
					<Input
						id="name"
						placeholder="Enter name or leave blank to use your display name"
						bind:value={name}
						disabled={isSubmitting}
					/>
				</div>

				<!-- Video Uploads -->
				<VideoUploadArea
					label="Front View Video"
					id="frontVideo"
					bind:file={frontVideo}
					disabled={isSubmitting}
					onchange={(e) => handleFileChange(e, 'front')}
				/>

				<VideoUploadArea
					label="Side View Video"
					id="sideVideo"
					bind:file={sideVideo}
					disabled={isSubmitting}
					onchange={(e) => handleFileChange(e, 'side')}
				/>

				<!-- Progress bar -->
				{#if isSubmitting && progress > 0}
					<div class="progress-container">
						<div class="progress-bar">
							<div 
								class="progress-fill"
								style="width: {progress}%"
							></div>
						</div>
						<p class="progress-text">
							Uploading... {progress}%
						</p>
					</div>
				{/if}

				<Button 
					type="submit" 
					class="submit-button" 
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

<style>
	.page-title {
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.2;
		letter-spacing: -0.025em;
	}

	.page-description {
		margin-top: 0.5rem;
		color: hsl(var(--muted-foreground));
	}

	.submit-form {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.progress-container {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.progress-bar {
		height: 0.5rem;
		width: 100%;
		overflow: hidden;
		border-radius: 9999px;
		background-color: hsl(var(--muted));
	}

	.progress-fill {
		height: 100%;
		background-color: hsl(var(--primary));
		transition: width 0.3s;
	}

	.progress-text {
		text-align: center;
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
	}

	@media (max-width: 640px) {
		.page-title {
			font-size: 1.5rem;
		}
	}
</style>

