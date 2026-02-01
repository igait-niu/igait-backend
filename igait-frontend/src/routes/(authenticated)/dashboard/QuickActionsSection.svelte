<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { ArrowRight, Upload, MessageSquare, History } from '@lucide/svelte';

	const actions = [
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
			title: 'View Submissions',
			description: 'See your past submissions and results',
			href: '/submissions',
			icon: History,
			variant: 'outline' as const
		}
	];
</script>

<section>
	<h2 class="section-heading">Quick Actions</h2>
	<div class="quick-actions-grid">
		{#each actions as action}
			<Card.Root class="action-card">
				<a href={action.href} class="action-link">
					<Card.Header>
						<div class="action-header">
							<div class="action-icon-wrapper">
								<svelte:component this={action.icon} class="h-6 w-6 text-primary" />
							</div>
							<div>
								<Card.Title class="action-title">{action.title}</Card.Title>
								<Card.Description>{action.description}</Card.Description>
							</div>
						</div>
					</Card.Header>
					<Card.Footer>
						<Button variant={action.variant} class="action-button">
							{action.title}
							<ArrowRight class="ml-2 h-4 w-4" />
						</Button>
					</Card.Footer>
				</a>
			</Card.Root>
		{/each}
	</div>
</section>

<style>
	.section-heading {
		font-size: 1.25rem;
		font-weight: 600;
		margin-bottom: var(--spacing-md);
	}

	.quick-actions-grid {
		display: grid;
		gap: var(--grid-gap);
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
	}

	:global(.action-card) {
		transition: all 0.2s;
		height: 100%;
	}

	:global(.action-card:hover) {
		transform: translateY(-4px);
		box-shadow: 0 10px 25px -5px hsl(var(--primary) / 0.15);
	}

	.action-link {
		display: flex;
		flex-direction: column;
		height: 100%;
		text-decoration: none;
		color: inherit;
	}

	.action-header {
		display: flex;
		gap: var(--spacing-md);
		align-items: flex-start;
	}

	.action-icon-wrapper {
		display: flex;
		height: 3rem;
		width: 3rem;
		flex-shrink: 0;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-md);
		background-color: hsl(var(--primary) / 0.1);
	}

	:global(.action-title) {
		font-size: 1.125rem;
	}

	:global(.action-button) {
		width: 100%;
	}
</style>
