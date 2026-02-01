<script lang="ts">
	import { getUser } from '$lib/hooks';
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { 
		Upload, 
		MessageSquare, 
		History, 
		ArrowRight,
		Activity,
		FileVideo,
		Clock
	} from '@lucide/svelte';

	// Get the authenticated user - guaranteed to exist in (authenticated) routes
	const user = getUser();

	const quickActions = [
		{
			title: 'New Submission',
			description: 'Upload walking videos for gait analysis',
			href: '/submit',
			icon: Upload,
			variant: 'default' as const
		},
		{
			title: 'AI Assistant',
			description: 'Chat with our AI about your results',
			href: '/assistant',
			icon: MessageSquare,
			variant: 'outline' as const
		},
		{
			title: 'View History',
			description: 'See your past submissions and results',
			href: '/history',
			icon: History,
			variant: 'outline' as const
		}
	];

	// Mock recent activity - would come from API in real implementation
	const recentActivity = [
		{ 
			id: 1, 
			type: 'submission', 
			status: 'completed', 
			date: 'Jan 28, 2026',
			description: 'Gait analysis completed'
		},
		{ 
			id: 2, 
			type: 'submission', 
			status: 'processing', 
			date: 'Jan 30, 2026',
			description: 'Video being processed'
		}
	];
</script>

<svelte:head>
	<title>Dashboard - iGait</title>
</svelte:head>

<div class="space-y-8">
	<!-- Welcome Section -->
	<section>
		<h1 class="text-3xl font-bold tracking-tight">
			Welcome back, {user.displayName.split(' ')[0]}!
		</h1>
		<p class="mt-2 text-muted-foreground">
			Here's an overview of your gait analysis activity.
		</p>
	</section>

	<!-- Quick Actions -->
	<section>
		<h2 class="mb-4 text-xl font-semibold">Quick Actions</h2>
		<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
			{#each quickActions as action}
				<Card.Root class="transition-colors hover:bg-muted/50">
					<a href={action.href} class="block">
						<Card.Header>
							<div class="flex items-center gap-4">
								<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
									<svelte:component this={action.icon} class="h-6 w-6 text-primary" />
								</div>
								<div>
									<Card.Title class="text-lg">{action.title}</Card.Title>
									<Card.Description>{action.description}</Card.Description>
								</div>
							</div>
						</Card.Header>
						<Card.Footer>
							<Button variant={action.variant} class="w-full">
								{action.title}
								<ArrowRight class="ml-2 h-4 w-4" />
							</Button>
						</Card.Footer>
					</a>
				</Card.Root>
			{/each}
		</div>
	</section>

	<!-- Stats Overview -->
	<section>
		<h2 class="mb-4 text-xl font-semibold">Your Activity</h2>
		<div class="grid gap-4 sm:grid-cols-3">
			<Card.Root>
				<Card.Header class="flex flex-row items-center justify-between pb-2">
					<Card.Title class="text-sm font-medium">Total Submissions</Card.Title>
					<FileVideo class="h-4 w-4 text-muted-foreground" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">12</div>
					<p class="text-xs text-muted-foreground">All time submissions</p>
				</Card.Content>
			</Card.Root>

			<Card.Root>
				<Card.Header class="flex flex-row items-center justify-between pb-2">
					<Card.Title class="text-sm font-medium">Analyses Complete</Card.Title>
					<Activity class="h-4 w-4 text-muted-foreground" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">10</div>
					<p class="text-xs text-muted-foreground">Successfully processed</p>
				</Card.Content>
			</Card.Root>

			<Card.Root>
				<Card.Header class="flex flex-row items-center justify-between pb-2">
					<Card.Title class="text-sm font-medium">In Progress</Card.Title>
					<Clock class="h-4 w-4 text-muted-foreground" />
				</Card.Header>
				<Card.Content>
					<div class="text-2xl font-bold">2</div>
					<p class="text-xs text-muted-foreground">Currently processing</p>
				</Card.Content>
			</Card.Root>
		</div>
	</section>

	<!-- Recent Activity -->
	<section>
		<div class="mb-4 flex items-center justify-between">
			<h2 class="text-xl font-semibold">Recent Activity</h2>
			<Button variant="ghost" size="sm" href="/history">
				View All
				<ArrowRight class="ml-2 h-4 w-4" />
			</Button>
		</div>
		
		<Card.Root>
			<Card.Content class="p-0">
				{#if recentActivity.length === 0}
					<div class="flex flex-col items-center justify-center py-12 text-center">
						<Activity class="mb-4 h-12 w-12 text-muted-foreground" />
						<h3 class="font-semibold">No activity yet</h3>
						<p class="text-sm text-muted-foreground">
							Submit your first walking video to get started!
						</p>
						<Button class="mt-4" href="/submit">
							<Upload class="mr-2 h-4 w-4" />
							New Submission
						</Button>
					</div>
				{:else}
					<div class="divide-y">
						{#each recentActivity as activity}
							<div class="flex items-center justify-between p-4">
								<div class="flex items-center gap-4">
									<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
										<FileVideo class="h-5 w-5 text-muted-foreground" />
									</div>
									<div>
										<p class="font-medium">{activity.description}</p>
										<p class="text-sm text-muted-foreground">{activity.date}</p>
									</div>
								</div>
								<Badge 
									variant={activity.status === 'completed' ? 'default' : 'secondary'}
								>
									{activity.status === 'completed' ? 'Complete' : 'Processing'}
								</Badge>
							</div>
						{/each}
					</div>
				{/if}
			</Card.Content>
		</Card.Root>
	</section>
</div>
