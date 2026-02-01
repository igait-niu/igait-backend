<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Users, Target, Heart } from '@lucide/svelte';
</script>

<svelte:head>
	<title>About - iGait</title>
</svelte:head>

<div class="py-12">
	<div class="mx-auto max-w-7xl px-4">
		<!-- Header -->
		<div class="text-center">
			<h1 class="text-4xl font-bold tracking-tight">About iGait</h1>
			<p class="mx-auto mt-4 max-w-2xl text-lg text-muted-foreground">
				Advancing gait analysis through artificial intelligence to improve lives.
			</p>
		</div>

		<!-- Mission Section -->
		<section class="mt-16">
			<div class="grid gap-8 md:grid-cols-3">
				<Card.Root>
					<Card.Header>
						<div class="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
							<Target class="h-6 w-6 text-primary" />
						</div>
						<Card.Title>Our Mission</Card.Title>
						<Card.Description>
							To make advanced gait analysis accessible to everyone through 
							cutting-edge AI technology and user-friendly interfaces.
						</Card.Description>
					</Card.Header>
				</Card.Root>

				<Card.Root>
					<Card.Header>
						<div class="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
							<Users class="h-6 w-6 text-primary" />
						</div>
						<Card.Title>Our Team</Card.Title>
						<Card.Description>
							A dedicated team of researchers, engineers, and healthcare 
							professionals working at Northern Illinois University.
						</Card.Description>
					</Card.Header>
				</Card.Root>

				<Card.Root>
					<Card.Header>
						<div class="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
							<Heart class="h-6 w-6 text-primary" />
						</div>
						<Card.Title>Our Impact</Card.Title>
						<Card.Description>
							Helping individuals understand their gait patterns for better 
							mobility awareness and early intervention.
						</Card.Description>
					</Card.Header>
				</Card.Root>
			</div>
		</section>

		<!-- Description -->
		<section class="mt-16">
			<Card.Root>
				<Card.Header>
					<Card.Title class="text-2xl">What is iGait?</Card.Title>
				</Card.Header>
				<Card.Content class="prose prose-slate max-w-none dark:prose-invert">
					<p>
						iGait is an advanced gait analysis system developed at Northern Illinois University. 
						Our platform uses state-of-the-art machine learning algorithms to analyze walking 
						patterns from simple video recordings.
					</p>
					<p>
						By uploading front and side view videos of walking, our system can extract detailed 
						pose estimation data and identify potential gait abnormalities that may indicate 
						underlying health conditions.
					</p>
					<p>
						Our goal is to make gait analysis accessible outside of specialized clinical 
						settings, enabling early detection and intervention for mobility-related issues.
					</p>
				</Card.Content>
			</Card.Root>
		</section>
	</div>
</div>
