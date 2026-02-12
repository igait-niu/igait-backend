<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Heart, Handshake } from '@lucide/svelte';

	const sponsors = [
		'Illinois Innovation Network',
		'NIH AIM-AHEAD',
		'NIU Office of Student Engagement and Experiential Learning',
		'NIU College of Health and Human Sciences',
		'SIUE Graduate School',
		'SIUE Undergraduate Research and Creative Activities'
	];

	const partners = [
		'NIU Center for Research Computing and Data',
		'NIU Center for the Interdisciplinary Study of Language and Literacy',
		'NIU Autism Caregiver Group',
		'NIU Speech-Language-Hearing Clinic',
		'NIU Division of Research and Innovation Partnerships',
		'NIU Foundation'
	];
</script>

<section class="content-section">
	<div class="section-header">
		<div class="section-icon">
			<Heart class="h-5 w-5 text-primary" />
		</div>
		<h2 class="section-title">Sponsors</h2>
	</div>
	<p class="section-intro">We are grateful to the following sponsors for their funding support:</p>

	<div class="sponsors-grid">
		{#each sponsors as sponsor}
			<Card.Root class="sponsor-card">
				<Card.Content class="sponsor-content">
					<span class="sponsor-name">{sponsor}</span>
				</Card.Content>
			</Card.Root>
		{/each}
	</div>
</section>

<section class="content-section">
	<div class="section-header">
		<div class="section-icon">
			<Handshake class="h-5 w-5 text-primary" />
		</div>
		<h2 class="section-title">Special Thanks</h2>
	</div>
	<p class="section-intro">We appreciate the continued support by the following partners:</p>

	<div class="partners-grid">
		{#each partners as partner}
			<div class="partner-item">
				<div class="partner-dot"></div>
				<span>{partner}</span>
			</div>
		{/each}
	</div>
</section>

<style>
	.content-section {
		margin-bottom: var(--spacing-xl);
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: var(--spacing-xs);
		margin-bottom: var(--spacing-lg);
	}

	.section-icon {
		display: inline-flex;
		height: 2.5rem;
		width: 2.5rem;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-md);
		background-color: hsl(var(--primary) / 0.1);
	}

	.section-title {
		font-size: 1.875rem;
		font-weight: 600;
	}

	.section-intro {
		color: hsl(var(--muted-foreground));
		margin-bottom: var(--spacing-lg);
	}

	.sponsors-grid {
		display: grid;
		gap: var(--spacing-md);
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
	}

	:global(.sponsor-card) {
		background: linear-gradient(135deg, hsl(var(--primary) / 0.05), hsl(var(--accent) / 0.05));
		border: 1px solid hsl(var(--border));
		transition: all 0.2s;
	}

	:global(.sponsor-card:hover) {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px hsl(var(--primary) / 0.1);
	}

	:global(.sponsor-content) {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: var(--spacing-lg);
		min-height: 100px;
	}

	.sponsor-name {
		text-align: center;
		font-weight: 500;
		line-height: 1.4;
	}

	.partners-grid {
		display: grid;
		gap: var(--spacing-md);
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
	}

	.partner-item {
		display: flex;
		align-items: flex-start;
		gap: var(--spacing-sm);
		padding: var(--spacing-sm);
		border-radius: var(--radius-md);
		transition: all 0.2s;
		animation: slideIn 0.3s ease-out;
	}

	.partner-item:hover {
		background-color: hsl(var(--muted) / 0.3);
	}

	.partner-dot {
		min-width: 0.5rem;
		height: 0.5rem;
		border-radius: 50%;
		background-color: hsl(var(--primary));
		margin-top: 0.5rem;
	}

	@keyframes slideIn {
		from {
			opacity: 0;
			transform: translateX(-10px);
		}
		to {
			opacity: 1;
			transform: translateX(0);
		}
	}
</style>
