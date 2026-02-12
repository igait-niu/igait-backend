<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Sparkles, Users, Heart } from '@lucide/svelte';

	const missions = [
		{
			icon: Sparkles,
			title: 'Our Mission',
			description:
				'To make advanced gait analysis accessible to everyone through cutting-edge AI technology and user-friendly interfaces.'
		},
		{
			icon: Users,
			title: 'Our Team',
			description:
				'A dedicated team of researchers, engineers, and healthcare professionals working across multiple Illinois universities.'
		},
		{
			icon: Heart,
			title: 'Our Impact',
			description:
				'Helping identify gait patterns for early intervention and improved outcomes for individuals and families.'
		}
	];
</script>

<section class="mission-section">
	<div class="mission-grid">
		{#each missions as mission}
			<Card.Root class="mission-card">
				<Card.Header>
					<div class="mission-icon">
						<svelte:component this={mission.icon} class="h-6 w-6 text-primary" />
					</div>
					<Card.Title>{mission.title}</Card.Title>
					<Card.Description class="mission-description">
						{mission.description}
					</Card.Description>
				</Card.Header>
			</Card.Root>
		{/each}
	</div>
</section>

<style>
	.mission-section {
		margin-bottom: var(--spacing-xl);
	}

	.mission-grid {
		display: grid;
		gap: var(--grid-gap);
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
	}

	:global(.mission-card) {
		transition: transform 0.2s;
	}

	:global(.mission-card:hover) {
		transform: translateY(-4px);
	}

	.mission-icon {
		display: inline-flex;
		height: 3rem;
		width: 3rem;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-md);
		background-color: hsl(var(--primary) / 0.1);
		margin-bottom: var(--spacing-md);
	}

	:global(.mission-description) {
		line-height: 1.6;
	}
</style>
