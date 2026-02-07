<script lang="ts">
	interface Props {
		checked?: boolean;
		disabled?: boolean;
		onCheckedChange?: (checked: boolean) => void;
	}

	let { checked = $bindable(false), disabled = false, onCheckedChange }: Props = $props();

	function toggle() {
		if (disabled) return;
		checked = !checked;
		onCheckedChange?.(checked);
	}
</script>

<button
	role="switch"
	aria-checked={checked}
	aria-label="Toggle"
	{disabled}
	class="switch"
	class:checked
	class:disabled
	onclick={toggle}
	type="button"
>
	<span class="thumb" class:checked></span>
</button>

<style>
	.switch {
		position: relative;
		display: inline-flex;
		align-items: center;
		width: 2.75rem;
		height: 1.5rem;
		border-radius: 9999px;
		border: none;
		padding: 0.125rem;
		cursor: pointer;
		transition: background-color 0.2s ease;
		background-color: hsl(var(--muted));
		flex-shrink: 0;
	}

	.switch.checked {
		background-color: hsl(var(--primary));
	}

	.switch.disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.thumb {
		display: block;
		width: 1.25rem;
		height: 1.25rem;
		border-radius: 9999px;
		background-color: hsl(var(--background));
		transition: transform 0.2s ease;
		transform: translateX(0);
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.thumb.checked {
		transform: translateX(1.25rem);
	}
</style>
