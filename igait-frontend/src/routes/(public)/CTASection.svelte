<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { ArrowRight } from '@lucide/svelte';
</script>

<section class="page-section">
	<div class="page-container">
		<div class="cta-card">
			<h2 class="cta-title">Ready to Get Started?</h2>
			<p class="cta-description">
				Create your free account today and start analyzing your gait patterns.
			</p>
			<Button 
				size="lg" 
				variant="secondary" 
				href="/signup"
				class="cta-button"
			>
				Create Free Account
				<ArrowRight class="ml-2 h-4 w-4" />
			</Button>
		</div>
	</div>
</section>

<style>
	.page-section {
		padding: var(--section-padding-y) 0;
	}

	.cta-card {
		background: linear-gradient(135deg, hsl(var(--primary) / 0.1), hsl(var(--primary) / 0.05));
		border-radius: var(--radius-lg);
		padding: var(--spacing-2xl);
		text-align: center;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: var(--spacing-md);
		transition: transform 0.2s, box-shadow 0.2s;
	}

	.cta-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 10px 25px -5px hsl(var(--primary) / 0.2);
	}

	.cta-title {
		font-size: 2rem;
		font-weight: bold;
	}

	.cta-description {
		font-size: 1.125rem;
		color: hsl(var(--muted-foreground));
		max-width: 36rem;
	}

	:global(.cta-button) {
		margin-top: var(--spacing-sm);
	}

	@media (max-width: 768px) {
		.cta-card {
			padding: var(--spacing-xl);
		}

		.cta-title {
			font-size: 1.5rem;
		}

		.cta-description {
			font-size: 1rem;
		}
	}
</style>
