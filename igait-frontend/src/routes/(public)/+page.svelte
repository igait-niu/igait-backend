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
<section class="relative overflow-hidden py-20 sm:py-32">
	<div class="mx-auto max-w-7xl px-4">
		<div class="text-center">
			<h1 class="text-4xl font-bold tracking-tight sm:text-6xl">
				AI-Powered
				<span class="text-primary"> Gait Analysis</span>
			</h1>
			<p class="mx-auto mt-6 max-w-2xl text-lg text-muted-foreground">
				Upload your walking videos and receive detailed analysis powered by machine learning. 
				Early detection of gait abnormalities can help identify potential health concerns.
			</p>
			<div class="mt-10 flex items-center justify-center gap-4">
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
<section class="border-t bg-muted/50 py-20">
	<div class="mx-auto max-w-7xl px-4">
		<h2 class="text-center text-3xl font-bold">How It Works</h2>
		<p class="mx-auto mt-4 max-w-2xl text-center text-muted-foreground">
			Our advanced pipeline processes your walking videos to provide detailed gait analysis.
		</p>
		
		<div class="mt-16 grid gap-8 sm:grid-cols-3">
			<Card.Root>
				<Card.Header>
					<div class="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
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
					<div class="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
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
					<div class="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
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
<section class="py-20">
	<div class="mx-auto max-w-7xl px-4">
		<div class="rounded-2xl bg-primary px-8 py-16 text-center text-primary-foreground">
			<h2 class="text-3xl font-bold">Ready to Get Started?</h2>
			<p class="mx-auto mt-4 max-w-xl text-primary-foreground/80">
				Create your free account today and start analyzing your gait patterns.
			</p>
			<Button 
				size="lg" 
				variant="secondary" 
				href="/signup"
				class="mt-8"
			>
				Create Free Account
				<ArrowRight class="ml-2 h-4 w-4" />
			</Button>
		</div>
	</div>
</section>
