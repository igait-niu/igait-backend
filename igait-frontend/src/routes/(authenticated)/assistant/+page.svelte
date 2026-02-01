<script lang="ts">
	import { getUser } from '$lib/hooks';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Construction, MessageSquare } from '@lucide/svelte';

	const user = getUser();
</script>

<svelte:head>
	<title>AI Assistant - iGait</title>
</svelte:head>

<div class="assistant-page stack-lg">
	<section>
		<h1 class="page-title">AI Assistant</h1>
		<p class="page-description">
			Chat with our AI assistant about your gait analysis results
		</p>
	</section>

	<!-- Coming Soon Placeholder -->
	<Card.Root class="coming-soon-card">
		<Card.Header>
			<div class="coming-soon-icon">
				<Construction class="h-8 w-8 text-primary" />
			</div>
			<Card.Title class="coming-soon-title">Coming Soon!</Card.Title>
			<Card.Description class="coming-soon-description">
				The AI Assistant feature is currently being built with love~ 🦊
			</Card.Description>
		</Card.Header>
		<Card.Content>
			<p class="coming-soon-text">
				Soon you'll be able to chat with our AI assistant to ask questions about 
				your gait analysis results, get personalized insights, and learn more about 
				the iGait system.
			</p>
			<div class="coming-soon-actions">
				<Button variant="outline" href="/dashboard">
					Back to Dashboard
				</Button>
				<Button href="/submit">
					<MessageSquare class="mr-2 h-4 w-4" />
					Make a Submission
				</Button>
			</div>
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

	:global(.coming-soon-card) {
		text-align: center;
	}

	.coming-soon-icon {
		margin: 0 auto 1rem;
		display: flex;
		height: 4rem;
		width: 4rem;
		align-items: center;
		justify-content: center;
		border-radius: 9999px;
		background-color: hsl(var(--primary) / 0.1);
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.7;
		}
	}

	:global(.coming-soon-title) {
		font-size: 1.5rem;
	}

	:global(.coming-soon-description) {
		font-size: 1rem;
	}

	.coming-soon-text {
		margin: 0 auto;
		max-width: 28rem;
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
	}

	.coming-soon-actions {
		margin-top: 1.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
	}
</style>
