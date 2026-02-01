<script lang="ts">
	import { getUser } from '$lib/hooks';
	import * as Card from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Button } from '$lib/components/ui/button';
	import { FileVideo, Mail, Send } from '@lucide/svelte';

	const user = getUser();

	// Mock historical data - in real implementation, this would come from API
	const submissions = [
		{ 
			id: 1, 
			date: 'Jan 28, 2026', 
			status: 'completed',
			result: 'Normal gait pattern detected'
		},
		{ 
			id: 2, 
			date: 'Jan 15, 2026', 
			status: 'completed',
			result: 'Minor asymmetry detected'
		},
		{ 
			id: 3, 
			date: 'Dec 20, 2025', 
			status: 'completed',
			result: 'Normal gait pattern detected'
		},
	];

	function handleEmailRequest() {
		const subject = encodeURIComponent('Request for Historical Submissions');
		const body = encodeURIComponent(
			`Hello iGAIT Support Team,\n\n` +
			`I would like to request my historical submissions from the iGAIT system.\n\n` +
			`User Email: ${user.email}\n` +
			`User ID: ${user.uid}\n\n` +
			`Thank you!`
		);
		window.location.href = `mailto:support@igaitapp.com?subject=${subject}&body=${body}`;
	}
</script>

<svelte:head>
	<title>History - iGait</title>
</svelte:head>

<div class="space-y-8">
	<section>
		<h1 class="text-3xl font-bold tracking-tight">Submission History</h1>
		<p class="mt-2 text-muted-foreground">
			View your past gait analysis submissions and results
		</p>
	</section>

	<!-- Recent Submissions -->
	<Card.Root>
		<Card.Header>
			<Card.Title>Recent Submissions</Card.Title>
			<Card.Description>
				Your most recent gait analysis submissions
			</Card.Description>
		</Card.Header>
		<Card.Content class="p-0">
			{#if submissions.length === 0}
				<div class="flex flex-col items-center justify-center py-12 text-center">
					<FileVideo class="mb-4 h-12 w-12 text-muted-foreground" />
					<h3 class="font-semibold">No submissions yet</h3>
					<p class="text-sm text-muted-foreground">
						You haven't made any gait analysis submissions yet.
					</p>
					<Button class="mt-4" href="/submit">
						Make Your First Submission
					</Button>
				</div>
			{:else}
				<div class="divide-y">
					{#each submissions as submission}
						<div class="flex items-center justify-between p-4">
							<div class="flex items-center gap-4">
								<div class="flex h-10 w-10 items-center justify-center rounded-full bg-muted">
									<FileVideo class="h-5 w-5 text-muted-foreground" />
								</div>
								<div>
									<p class="font-medium">{submission.result}</p>
									<p class="text-sm text-muted-foreground">{submission.date}</p>
								</div>
							</div>
							<Badge variant={submission.status === 'completed' ? 'default' : 'secondary'}>
								{submission.status}
							</Badge>
						</div>
					{/each}
				</div>
			{/if}
		</Card.Content>
	</Card.Root>

	<!-- Request Full History -->
	<Card.Root>
		<Card.Header>
			<div class="flex items-center gap-4">
				<div class="flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
					<Mail class="h-6 w-6 text-primary" />
				</div>
				<div>
					<Card.Title>Need More History?</Card.Title>
					<Card.Description>
						Request a complete record of your submissions via email
					</Card.Description>
				</div>
			</div>
		</Card.Header>
		<Card.Content>
			<p class="mb-4 text-sm text-muted-foreground">
				For a complete history of your submissions including original videos and 
				detailed analysis reports, please contact our support team.
			</p>
			<Button onclick={handleEmailRequest}>
				<Send class="mr-2 h-4 w-4" />
				Email Support Team
			</Button>
		</Card.Content>
	</Card.Root>
</div>
