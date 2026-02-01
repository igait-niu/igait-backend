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
	import FormSelect from './FormSelect.svelte';
	import type { Ethnicity, Sex, UserRole } from '$lib/api/types';

	const user = getUser();

	// Demographic fields
	let age = $state('');
	let sex = $state<Sex | ''>('');
	let ethnicity = $state<Ethnicity | ''>('');
	let heightFeet = $state('');
	let heightInches = $state('');
	let weight = $state('');
	let role = $state<UserRole | ''>('');

	// Video fields
	let frontVideo: File | undefined = $state(undefined);
	let sideVideo: File | undefined = $state(undefined);
	
	// Form state
	let isSubmitting = $state(false);
	let progress = $state(0);
	let error: Option<AppError> = $state(None());

	// Select options
	const sexOptions = [
		{ value: 'M', label: 'Male' },
		{ value: 'F', label: 'Female' },
		{ value: 'O', label: 'Other' }
	];

	const ethnicityOptions = [
		{ value: 'africanAmerican', label: 'African American/Black' },
		{ value: 'nativeAmerican', label: 'Native American/American Indian' },
		{ value: 'asian', label: 'Asian' },
		{ value: 'hispanic', label: 'Hispanic/Latino' },
		{ value: 'caucasian', label: 'Caucasian/White' },
		{ value: 'pacificIslander', label: 'Pacific Islander' }
	];

	const roleOptions = [
		{ value: 'parent', label: 'Parent' },
		{ value: 'doctor', label: 'Medical Professional' },
		{ value: 'schoolOfficial', label: 'School Official' },
		{ value: 'sibling', label: 'Sibling' },
		{ value: 'grandparent', label: 'Grandparent' },
		{ value: 'self', label: 'Self' }
	];

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

	// Validation helpers
	const isFormValid = $derived(
		age !== '' && 
		sex !== '' && 
		ethnicity !== '' && 
		heightFeet !== '' && 
		heightInches !== '' && 
		weight !== '' && 
		role !== '' && 
		frontVideo !== undefined && 
		sideVideo !== undefined
	);

	async function handleSubmit(e: Event) {
		e.preventDefault();
		error = None();

		if (!frontVideo || !sideVideo) {
			error = Some(new AppError('Please select both front and side videos'));
			return;
		}

		if (!sex || !ethnicity || !role) {
			error = Some(new AppError('Please fill out all required fields'));
			return;
		}

		isSubmitting = true;
		progress = 0;

		const request: ContributionRequest = {
			uid: user.uid,
			email: user.email,
			age: parseInt(age, 10),
			sex: sex as Sex,
			ethnicity: ethnicity as Ethnicity,
			heightFeet: parseInt(heightFeet, 10),
			heightInches: parseInt(heightInches, 10),
			weight: parseInt(weight, 10),
			role: role as UserRole,
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
				<!-- Patient Information Section -->
				<fieldset class="form-section">
					<legend class="form-section__title">Patient Information</legend>
					
					<div class="form-row form-row--two-col">
						<div class="form-group">
							<Label for="age">Age *</Label>
							<Input
								id="age"
								type="number"
								min="1"
								max="115"
								placeholder="Enter age"
								bind:value={age}
								disabled={isSubmitting}
								required
							/>
						</div>

						<FormSelect
							label="Sex"
							id="sex"
							bind:value={sex}
							options={sexOptions}
							placeholder="Select sex"
							required
							disabled={isSubmitting}
						/>
					</div>

					<FormSelect
						label="Ethnicity"
						id="ethnicity"
						bind:value={ethnicity}
						options={ethnicityOptions}
						placeholder="Select ethnicity"
						required
						disabled={isSubmitting}
					/>

					<div class="form-row form-row--height">
						<div class="form-group">
							<Label for="heightFeet">Height (feet) *</Label>
							<Input
								id="heightFeet"
								type="number"
								min="1"
								max="8"
								placeholder="Feet"
								bind:value={heightFeet}
								disabled={isSubmitting}
								required
							/>
						</div>
						<div class="form-group">
							<Label for="heightInches">Height (inches) *</Label>
							<Input
								id="heightInches"
								type="number"
								min="0"
								max="11"
								placeholder="Inches"
								bind:value={heightInches}
								disabled={isSubmitting}
								required
							/>
						</div>
					</div>

					<div class="form-group">
						<Label for="weight">Weight (lbs) *</Label>
						<Input
							id="weight"
							type="number"
							min="1"
							max="500"
							placeholder="Enter weight in pounds"
							bind:value={weight}
							disabled={isSubmitting}
							required
						/>
					</div>
				</fieldset>

				<!-- Submitter Information Section -->
				<fieldset class="form-section">
					<legend class="form-section__title">Submitter Information</legend>
					
					<FormSelect
						label="Your Role"
						id="role"
						bind:value={role}
						options={roleOptions}
						placeholder="Select your relationship to patient"
						required
						disabled={isSubmitting}
					/>
				</fieldset>

				<!-- Video Uploads Section -->
				<fieldset class="form-section">
					<legend class="form-section__title">Video Uploads</legend>
					
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
				</fieldset>

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
					disabled={isSubmitting || !isFormValid}
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
		gap: 2rem;
	}

	.form-section {
		border: none;
		padding: 0;
		margin: 0;
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.form-section__title {
		font-size: 1rem;
		font-weight: 600;
		color: hsl(var(--foreground));
		padding-bottom: 0.5rem;
		border-bottom: 1px solid hsl(var(--border));
		margin-bottom: 0.5rem;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.form-row {
		display: grid;
		gap: 1rem;
	}

	.form-row--two-col {
		grid-template-columns: 1fr 1fr;
	}

	.form-row--height {
		grid-template-columns: 1fr 1fr;
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

		.form-row--two-col,
		.form-row--height {
			grid-template-columns: 1fr;
		}
	}
</style>

