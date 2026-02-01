<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Activity, Shield, Zap } from '@lucide/svelte';

	const features = [
		{
			icon: Activity,
			title: 'Upload Videos',
			description: 'Simply upload front and side view videos of walking. Our system handles the rest.'
		},
		{
			icon: Zap,
			title: 'AI Processing',
			description: 'Our ML pipeline extracts pose data and analyzes gait patterns with high accuracy.'
		},
		{
			icon: Shield,
			title: 'Get Results',
			description: 'Receive detailed analysis results via email with actionable insights.'
		}
	];
</script>

<section class="features-section">
	<div class="page-container">
		<h2 class="section-title">How It Works</h2>
		<p class="section-description">
			Our advanced pipeline processes your walking videos to provide detailed gait analysis.
		</p>
		
		<div class="features-grid">
			{#each features as feature}
				<Card.Root>
					<Card.Header>
						<div class="feature-icon">
							<svelte:component this={feature.icon} class="h-6 w-6 text-primary" />
						</div>
						<Card.Title>{feature.title}</Card.Title>
						<Card.Description>
							{feature.description}
						</Card.Description>
					</Card.Header>
				</Card.Root>
			{/each}
		</div>
	</div>
</section>

<style>
	.features-section {
		background-color: hsl(var(--muted) / 0.3);
		padding: var(--section-padding-y) 0;
	}

	.section-title {
		font-size: 2.25rem;
		font-weight: bold;
		text-align: center;
		margin-bottom: var(--spacing-sm);
	}

	.section-description {
		text-align: center;
		font-size: 1.125rem;
		color: hsl(var(--muted-foreground));
		margin-bottom: var(--spacing-xl);
		max-width: 42rem;
		margin-left: auto;
		margin-right: auto;
	}

	.features-grid {
		display: grid;
		gap: var(--grid-gap);
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
	}

	.feature-icon {
		display: inline-flex;
		height: 3rem;
		width: 3rem;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-md);
		background-color: hsl(var(--primary) / 0.1);
		margin-bottom: var(--spacing-md);
	}

	@media (max-width: 768px) {
		.section-title {
			font-size: 1.875rem;
		}

		.section-description {
			font-size: 1rem;
		}
	}
</style>
