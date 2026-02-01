<script lang="ts">
	import { Label } from '$lib/components/ui/label';
	import { Video, CheckCircle } from '@lucide/svelte';

	interface Props {
		label: string;
		id: string;
		file: File | undefined;
		disabled: boolean;
		onchange: (e: Event) => void;
	}

	let { label, id, file = $bindable(), disabled, onchange }: Props = $props();
</script>

<div class="form-group">
	<Label for={id}>{label}</Label>
	<div class="upload-container">
		<label for={id} class="upload-area" class:has-file={file} class:disabled>
			<input
				{id}
				type="file"
				accept="video/*"
				class="hidden"
				{onchange}
				{disabled}
			/>
			{#if file}
				<CheckCircle class="upload-icon-success" />
				<span class="upload-filename">{file.name}</span>
				<span class="upload-filesize">
					{(file.size / 1024 / 1024).toFixed(1)} MB
				</span>
			{:else}
				<Video class="upload-icon" />
				<span class="upload-placeholder">Click to upload {label.toLowerCase()}</span>
			{/if}
		</label>
	</div>
</div>

<style>
	.form-group {
		display: flex;
		flex-direction: column;
		gap: var(--spacing-xs);
	}

	.upload-container {
		position: relative;
	}

	.upload-area {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--spacing-2xl);
		border: 2px dashed hsl(var(--border));
		border-radius: var(--radius-lg);
		cursor: pointer;
		transition: all 0.2s;
		background-color: hsl(var(--muted) / 0.3);
		min-height: 180px;
		gap: var(--spacing-sm);
	}

	.upload-area:hover:not(.disabled) {
		border-color: hsl(var(--primary));
		background-color: hsl(var(--primary) / 0.05);
	}

	.upload-area.has-file {
		border-color: hsl(var(--primary));
		background-color: hsl(var(--primary) / 0.1);
	}

	.upload-area.disabled {
		opacity: 0.6;
		cursor: not-allowed;
	}

	:global(.upload-icon) {
		width: 3rem;
		height: 3rem;
		color: hsl(var(--muted-foreground));
	}
	

	:global(.upload-icon-success) {
		width: 3rem;
		height: 3rem;
		color: hsl(var(--primary));
	}

	.upload-placeholder {
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
	}

	.upload-filename {
		font-weight: 500;
		text-align: center;
		word-break: break-word;
	}

	.upload-filesize {
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
	}

	.hidden {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border-width: 0;
	}
</style>
