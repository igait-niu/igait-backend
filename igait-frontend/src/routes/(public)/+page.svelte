<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import * as Card from '$lib/components/ui/card';
	import { ArrowRight, Activity, Shield, Zap } from '@lucide/svelte';
</script>

<svelte:head>
	<title>iGait - AI-Powered Gait Analysis</title>
	<meta name="description" content="Advanced gait analysis using artificial intelligence for early detection of mobility issues." />
</svelte:head>

<!-- Hero Section -->
<section class="hero-section">
	<div class="page-container">
		<div class="hero-content">
			<h1 class="hero-title">
				AI-Powered
				<span class="text-primary"> Gait Analysis</span>
			</h1>
			<p class="hero-description">
				Upload your walking videos and receive detailed analysis powered by machine learning. 
				Early detection of gait abnormalities can help identify potential health concerns.
			</p>
			<div class="hero-actions">
				<Button size="lg" href="/signup">
					Get Started
					<ArrowRight class="ml-2 h-4 w-4" />
				</Button>
				<Button size="lg" variant="outline" href="/about">
					Learn More
				</Button>
			</div>
		</div>
	</div>
</section>

<!-- Features Section -->
<section class="features-section">
	<div class="page-container">
		<h2 class="section-title">How It Works</h2>
		<p class="section-description">
			Our advanced pipeline processes your walking videos to provide detailed gait analysis.
		</p>
		
		<div class="features-grid">
			<Card.Root>
				<Card.Header>
					<div class="feature-icon">
						<Activity class="h-6 w-6 text-primary" />
					</div>
					<Card.Title>Upload Videos</Card.Title>
					<Card.Description>
						Simply upload front and side view videos of walking. Our system handles the rest.
					</Card.Description>
				</Card.Header>
			</Card.Root>
			
			<Card.Root>
				<Card.Header>
					<div class="feature-icon">
						<Zap class="h-6 w-6 text-primary" />
					</div>
					<Card.Title>AI Processing</Card.Title>
					<Card.Description>
						Our ML pipeline extracts pose data and analyzes gait patterns with high accuracy.
					</Card.Description>
				</Card.Header>
			</Card.Root>
			
			<Card.Root>
				<Card.Header>
					<div class="feature-icon">
						<Shield class="h-6 w-6 text-primary" />
					</div>
					<Card.Title>Get Results</Card.Title>
					<Card.Description>
						Receive detailed analysis results via email with actionable insights.
					</Card.Description>
				</Card.Header>
			</Card.Root>
		</div>
	</div>
</section>

<!-- CTA Section -->
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
	/* Hero Section */
	.hero-section {
		position: relative;
		overflow: hidden;
		padding-top: var(--section-padding-y);
		padding-bottom: var(--section-padding-y);
	}

	@media (min-width: 640px) {
		.hero-section {
			padding-top: 8rem;
			padding-bottom: 8rem;
		}
	}

	.hero-content {
		text-align: center;
	}

	.hero-title {
		font-size: 2.25rem;
		font-weight: 700;
		line-height: 1.1;
		letter-spacing: -0.025em;
	}

	@media (min-width: 640px) {
		.hero-title {
			font-size: 3.75rem;
		}
	}

	.hero-description {
		margin-left: auto;
		margin-right: auto;
		margin-top: 1.5rem;
		max-width: var(--container-narrow);
		font-size: 1.125rem;
		line-height: 1.75rem;
		color: hsl(var(--muted-foreground));
	}

	.hero-actions {
		margin-top: 2.5rem;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
	}

	/* Features Section */
	.features-section {
		border-top: 1px solid hsl(var(--border));
		background-color: hsl(var(--muted) / 0.5);
		padding-top: var(--section-padding-y);
		padding-bottom: var(--section-padding-y);
	}

	.section-title {
		text-align: center;
		font-size: 1.875rem;
		font-weight: 700;
	}

	.section-description {
		margin-left: auto;
		margin-right: auto;
		margin-top: 1rem;
		max-width: var(--container-narrow);
		text-align: center;
		color: hsl(var(--muted-foreground));
	}

	.features-grid {
		margin-top: 4rem;
		display: grid;
		gap: var(--grid-gap);
		grid-template-columns: 1fr;
	}

	@media (min-width: 640px) {
		.features-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.feature-icon {
		margin-bottom: 1rem;
		display: inline-flex;
		height: 3rem;
		width: 3rem;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-lg);
		background-color: hsl(var(--primary) / 0.1);
	}

	/* CTA Section */
	.cta-card {
		border-radius: 1rem;
		background-color: hsl(var(--primary));
		padding: 4rem 2rem;
		text-align: center;
		color: hsl(var(--primary-foreground));
	}

	.cta-title {
		font-size: 1.875rem;
		font-weight: 700;
	}

	.cta-description {
		margin-left: auto;
		margin-right: auto;
		margin-top: 1rem;
		max-width: 36rem;
		color: hsl(var(--primary-foreground) / 0.8);
	}
</style>
