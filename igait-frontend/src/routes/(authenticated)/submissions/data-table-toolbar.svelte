<script lang="ts">
	import { Input } from '$lib/components/ui/input';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { X, ListFilter } from '@lucide/svelte';

	type Props = {
		statusFilter: string;
		searchQuery: string;
		onStatusFilterChange: (value: string) => void;
		onSearchChange: (value: string) => void;
		onReset: () => void;
		hasActiveFilters: boolean;
	};

	let { statusFilter, searchQuery, onStatusFilterChange, onSearchChange, onReset, hasActiveFilters }: Props = $props();

	function getStatusLabel(value: string): string {
		switch (value) {
			case 'completed':
				return 'Completed';
			case 'processing':
				return 'Processing';
			case 'error':
				return 'Error';
			default:
				return 'All Status';
		}
	}
</script>

<div class="flex items-center justify-between">
	<div class="flex flex-1 items-center space-x-2">
		<Input
			placeholder="Filter submissions..."
			value={searchQuery}
			oninput={(e) => onSearchChange(e.currentTarget.value)}
			class="h-8 w-[150px] lg:w-[250px]"
		/>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				<Button variant="outline" size="sm" class="h-8 border-dashed">
					<ListFilter class="mr-2 h-4 w-4" />
					{getStatusLabel(statusFilter)}
					{#if statusFilter !== 'all'}
						<span class="ml-2 rounded-sm bg-primary px-1 py-0.5 text-xs font-semibold text-primary-foreground">
							1
						</span>
					{/if}
				</Button>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="start">
				<DropdownMenu.Label>Filter by status</DropdownMenu.Label>
				<DropdownMenu.Separator />
				<DropdownMenu.Item onclick={() => onStatusFilterChange('all')}>
					All Status
				</DropdownMenu.Item>
				<DropdownMenu.Item onclick={() => onStatusFilterChange('completed')}>
					Completed
				</DropdownMenu.Item>
				<DropdownMenu.Item onclick={() => onStatusFilterChange('processing')}>
					Processing
				</DropdownMenu.Item>
				<DropdownMenu.Item onclick={() => onStatusFilterChange('error')}>
					Error
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
		{#if hasActiveFilters}
			<Button variant="ghost" onclick={onReset} class="h-8 px-2 lg:px-3">
				Reset
				<X class="ml-2 h-4 w-4" />
			</Button>
		{/if}
	</div>
</div>
