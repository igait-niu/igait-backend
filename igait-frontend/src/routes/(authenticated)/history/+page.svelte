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

<div class="stack-lg">
	<section>
		<h1 class="page-title">Submission History</h1>
		<p class="page-description">
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
				<div class="empty-state">
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
				<div class="history-list">
					{#each submissions as submission}
						<div class="history-item">
							<div class="history-content">
								<div class="history-icon">
									<FileVideo class="h-5 w-5 text-muted-foreground" />
								</div>
								<div>
									<p class="history-result">{submission.result}</p>
									<p class="history-date">{submission.date}</p>
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
			<div class="request-header">
				<div class="request-icon">
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
			<p class="request-description">
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

<style>
	.page-title {
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.2;
		letter-spacing: -0.025em;
	}

	@media (min-width: 640px) {
		.page-title {
			font-size: 1.875rem;
		}
	}

	.page-description {
		margin-top: 0.5rem;
		color: hsl(var(--muted-foreground));
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: var(--spacing-xl) 0;
		text-align: center;
	}

	:global(.empty-icon) {
		margin-bottom: 1rem;
		height: 3rem;
		width: 3rem;
		color: hsl(var(--muted-foreground));
	}

	:global(.empty-title) {
		font-weight: 600;
	}

	:global(.empty-description) {
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
		margin-bottom: 1rem;
	}

	.history-list {
		border-top: 1px solid hsl(var(--border));
	}

	.history-list > :not(:last-child) {
		border-bottom: 1px solid hsl(var(--border));
	}

	.history-item {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 1rem;
	}

	.history-content {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.history-icon {
		display: flex;
		height: 2.5rem;
		width: 2.5rem;
		align-items: center;
		justify-content: center;
		border-radius: 9999px;
		background-color: hsl(var(--muted));
	}

	.history-result {
		font-weight: 500;
	}

	.history-date {
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
	}

	.request-header {
		display: flex;
		align-items: center;
		gap: 1rem;
	}

	.request-icon {
		display: flex;
		height: 3rem;
		width: 3rem;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-lg);
		background-color: hsl(var(--primary) / 0.1);
	}

	.request-description {
		margin-bottom: 1rem;
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
	}
</style>
