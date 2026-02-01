<script lang="ts">
	import * as Table from '$lib/components/ui/table';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { ArrowUpDown, MoreHorizontal, Download, Eye } from '@lucide/svelte';
	import DataTableToolbar from './data-table-toolbar.svelte';
	import JobDetailsDialog from './JobDetailsDialog.svelte';
	import type { Job } from '../../../types/Job';
	import type { JobStatus } from '../../../types/JobStatus';

	type Props = {
		data: Job[];
		initialStatusFilter?: string;
	};

	let { data, initialStatusFilter }: Props = $props();

	// Create jobs with indices (job IDs)
	type JobWithId = Job & { id: number };
	const jobsWithIds = $derived(data.map((job, index) => ({ ...job, id: index })));

	let sortColumn: 'date' | 'status' | null = $state(null);
	let sortDirection: 'asc' | 'desc' = $state('desc');
	let searchQuery: string = $state('');
	let selectedJob: Job | null = $state(null);
	
	const statusFilter: string = $derived(initialStatusFilter || 'all');
	let manualStatusFilter: string | null = $state(null);
	const effectiveStatusFilter = $derived(manualStatusFilter ?? statusFilter);

	// Helper to get status display info
	function getStatusInfo(status: JobStatus) {
		switch (status.code) {
			case 'Complete':
				return { 
					label: status.asd ? 'ASD Detected' : 'No ASD', 
					variant: status.asd ? 'destructive' as const : 'default' as const 
				};
			case 'Error':
				return { label: 'Error', variant: 'destructive' as const };
			case 'Processing':
				return { 
					label: `Stage ${status.stage}/${status.num_stages}`, 
					variant: 'secondary' as const 
				};
			case 'Submitted':
			default:
				return { label: 'Submitted', variant: 'outline' as const };
		}
	}

	// Filter and sort data
	const filteredData = $derived.by(() => {
		let filtered = jobsWithIds;

		// Apply status filter
		if (effectiveStatusFilter !== 'all') {
			filtered = filtered.filter((job) => {
				if (effectiveStatusFilter === 'completed') return job.status.code === 'Complete';
				if (effectiveStatusFilter === 'error') return job.status.code === 'Error';
				if (effectiveStatusFilter === 'processing')
					return job.status.code === 'Processing' || job.status.code === 'Submitted';
				return true;
			});
		}

		// Apply search filter
		if (searchQuery) {
			const query = searchQuery.toLowerCase();
			filtered = filtered.filter((job) => {
				return (
					job.id.toString().includes(query) ||
					job.status.value.toLowerCase().includes(query) ||
					getStatusInfo(job.status).label.toLowerCase().includes(query) ||
					new Date(job.timestamp * 1000).toLocaleDateString().includes(query)
				);
			});
		}

		// Apply sorting
		if (sortColumn) {
			filtered = [...filtered].sort((a, b) => {
				let aVal: any, bVal: any;

				if (sortColumn === 'date') {
					aVal = a.timestamp;
					bVal = b.timestamp;
				} else if (sortColumn === 'status') {
					// Sort order: Error > Processing > Submitted > Complete
					const statusOrder = { 'Error': 0, 'Processing': 1, 'Submitted': 2, 'Complete': 3 };
					aVal = statusOrder[a.status.code] ?? 2;
					bVal = statusOrder[b.status.code] ?? 2;
				}

				if (sortDirection === 'asc') {
					return aVal > bVal ? 1 : -1;
				} else {
					return aVal < bVal ? 1 : -1;
				}
			});
		}

		return filtered;
	});

	function toggleSort(column: 'date' | 'status') {
		if (sortColumn === column) {
			sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
		} else {
			sortColumn = column;
			sortDirection = 'desc';
		}
	}

	function handleStatusFilterChange(value: string) {
		manualStatusFilter = value;
	}

	function handleSearchChange(value: string) {
		searchQuery = value;
	}

	function resetFilters() {
		manualStatusFilter = null;
		searchQuery = '';
		sortColumn = null;
	}

	function viewJobDetails(job: Job) {
		selectedJob = job;
	}

	function closeJobDetails() {
		selectedJob = null;
	}
</script>

<div class="space-y-4">
	<DataTableToolbar
		statusFilter={effectiveStatusFilter}
		{searchQuery}
		onStatusFilterChange={handleStatusFilterChange}
		onSearchChange={handleSearchChange}
		onReset={resetFilters}
		hasActiveFilters={effectiveStatusFilter !== 'all' || searchQuery !== ''}
	/>
	<div class="rounded-md border">
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head class="w-[80px]">Job ID</Table.Head>
					<Table.Head class="w-[300px]">Description</Table.Head>
					<Table.Head>
						<Button
							variant="ghost"
							size="sm"
							class="-ml-3 h-8"
							onclick={() => toggleSort('status')}
						>
							Status
							<ArrowUpDown class="ml-2 h-4 w-4" />
						</Button>
					</Table.Head>
					<Table.Head>
						<Button
							variant="ghost"
							size="sm"
							class="-ml-3 h-8"
							onclick={() => toggleSort('date')}
						>
							Date
							<ArrowUpDown class="ml-2 h-4 w-4" />
						</Button>
					</Table.Head>
					<Table.Head class="w-[50px]"></Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#if filteredData.length > 0}
					{#each filteredData as job, index}
						{@const statusInfo = getStatusInfo(job.status)}
						<Table.Row>						<Table.Cell class="font-mono text-sm text-muted-foreground">{job.id}</Table.Cell>						<Table.Cell class="font-medium">{job.status.value}</Table.Cell>
							<Table.Cell>
								<Badge variant={statusInfo.variant}>{statusInfo.label}</Badge>
							</Table.Cell>
							<Table.Cell>
							{new Date(job.timestamp * 1000).toLocaleDateString('en-US', {
									month: 'short',
									day: 'numeric',
									year: 'numeric'
								})}
							</Table.Cell>
							<Table.Cell>
								<DropdownMenu.Root>
									<DropdownMenu.Trigger>
										<Button variant="ghost" class="h-8 w-8 p-0">
											<span class="sr-only">Open menu</span>
											<MoreHorizontal class="h-4 w-4" />
										</Button>
									</DropdownMenu.Trigger>
									<DropdownMenu.Content align="end">
										<DropdownMenu.Label>Actions</DropdownMenu.Label>
										<DropdownMenu.Separator />
										<DropdownMenu.Item onclick={() => viewJobDetails(job)}>
											<Eye class="mr-2 h-4 w-4" />
											View details
										</DropdownMenu.Item>
										{#if job.status.code === 'Complete'}
											<DropdownMenu.Item>
												<Download class="mr-2 h-4 w-4" />
												Download results
											</DropdownMenu.Item>
										{/if}
									</DropdownMenu.Content>
								</DropdownMenu.Root>
							</Table.Cell>
						</Table.Row>
					{/each}
				{:else}
					<Table.Row>
						<Table.Cell colspan={5} class="h-24 text-center"> No results. </Table.Cell>
					</Table.Row>
				{/if}
			</Table.Body>
		</Table.Root>
	</div>
	
	<!-- Results count -->
	<div class="flex items-center justify-between text-sm text-muted-foreground">
		<div>
			{filteredData.length} of {data.length} submission(s)
		</div>
	</div>
</div>

{#if selectedJob}
	<JobDetailsDialog job={selectedJob} onClose={closeJobDetails} />
{/if}
