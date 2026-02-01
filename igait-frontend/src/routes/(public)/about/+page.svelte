<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Users, Heart, GraduationCap, Building2, Handshake, Sparkles } from '@lucide/svelte';
</script>

<svelte:head>
	<title>About - iGait</title>
	<meta name="description" content="Learn about the iGait research team advancing early autism detection through innovative gait analysis technology." />
</svelte:head>

<div class="py-12">
	<div class="mx-auto max-w-7xl px-4">
		<!-- Hero Header -->
		<div class="text-center">
			<Badge variant="secondary" class="mb-4">About Us</Badge>
			<h1 class="text-4xl font-bold tracking-tight sm:text-5xl">About iGait</h1>
			<p class="mx-auto mt-4 max-w-3xl text-lg text-muted-foreground">
				Advancing early autism detection through innovative technology and collaborative research
			</p>
		</div>

		<!-- Mission Cards -->
		<section class="mt-16">
			<div class="grid gap-6 md:grid-cols-3">
				<Card.Root class="border-2 hover:border-primary/50 transition-colors">
					<Card.Header>
						<div class="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
							<Sparkles class="h-6 w-6 text-primary" />
						</div>
						<Card.Title>Our Mission</Card.Title>
						<Card.Description class="text-base">
							To make advanced gait analysis accessible to everyone through 
							cutting-edge AI technology and user-friendly interfaces.
						</Card.Description>
					</Card.Header>
				</Card.Root>

				<Card.Root class="border-2 hover:border-primary/50 transition-colors">
					<Card.Header>
						<div class="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
							<Users class="h-6 w-6 text-primary" />
						</div>
						<Card.Title>Our Team</Card.Title>
						<Card.Description class="text-base">
							A dedicated team of researchers, engineers, and healthcare 
							professionals working across multiple Illinois universities.
						</Card.Description>
					</Card.Header>
				</Card.Root>

				<Card.Root class="border-2 hover:border-primary/50 transition-colors">
					<Card.Header>
						<div class="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
							<Heart class="h-6 w-6 text-primary" />
						</div>
						<Card.Title>Our Impact</Card.Title>
						<Card.Description class="text-base">
							Helping identify gait patterns for early intervention and 
							improved outcomes for individuals and families.
						</Card.Description>
					</Card.Header>
				</Card.Root>
			</div>
		</section>

		<!-- Student Research Team Section -->
		<section class="mt-20">
			<div class="flex items-center gap-3 mb-8">
				<div class="inline-flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
					<GraduationCap class="h-5 w-5 text-primary" />
				</div>
				<h2 class="text-2xl font-bold">Student Research Team</h2>
			</div>

			<div class="grid gap-8 lg:grid-cols-2">
				<!-- NIU -->
				<Card.Root>
					<Card.Header>
						<Card.Title class="flex items-center gap-2">
							<Building2 class="h-5 w-5 text-muted-foreground" />
							Northern Illinois University (NIU)
						</Card.Title>
					</Card.Header>
					<Card.Content class="space-y-6">
						<div>
							<h4 class="font-semibold text-sm text-muted-foreground uppercase tracking-wide mb-3">
								Graduate Students
							</h4>
							<div class="flex flex-wrap gap-2">
								{#each ['Gabriela Ibarra', 'Mahesh Raju', 'Ricardo Torres', 'Alicia LaRouech', 'Viviana Cortes', 'Noelle Veome'] as name}
									<Badge variant="secondary">{name}</Badge>
								{/each}
							</div>
						</div>
						<Separator />
						<div>
							<h4 class="font-semibold text-sm text-muted-foreground uppercase tracking-wide mb-3">
								Undergraduate Students
							</h4>
							<div class="flex flex-wrap gap-2">
								{#each ['John White', 'Michael Sensenbrenner', 'Luke Ali', 'Angelica Sanyal', 'Zoey Proper', 'Rachel Conolly'] as name}
									<Badge variant="outline">{name}</Badge>
								{/each}
							</div>
						</div>
					</Card.Content>
				</Card.Root>

				<!-- SIUE -->
				<Card.Root>
					<Card.Header>
						<Card.Title class="flex items-center gap-2">
							<Building2 class="h-5 w-5 text-muted-foreground" />
							Southern Illinois University Edwardsville (SIUE)
						</Card.Title>
					</Card.Header>
					<Card.Content class="space-y-6">
						<div>
							<h4 class="font-semibold text-sm text-muted-foreground uppercase tracking-wide mb-3">
								Graduate Students
							</h4>
							<div class="flex flex-wrap gap-2">
								{#each ['Amit Shrestha', 'Prashanna Pandit'] as name}
									<Badge variant="secondary">{name}</Badge>
								{/each}
							</div>
						</div>
						<Separator />
						<div>
							<h4 class="font-semibold text-sm text-muted-foreground uppercase tracking-wide mb-3">
								Undergraduate Students
							</h4>
							<div class="flex flex-wrap gap-2">
								{#each ['Cody Schaefer', 'Vinhhy Pham'] as name}
									<Badge variant="outline">{name}</Badge>
								{/each}
							</div>
						</div>
					</Card.Content>
				</Card.Root>
			</div>
		</section>

		<!-- Sponsors Section -->
		<section class="mt-20">
			<div class="flex items-center gap-3 mb-4">
				<div class="inline-flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
					<Heart class="h-5 w-5 text-primary" />
				</div>
				<h2 class="text-2xl font-bold">Sponsors</h2>
			</div>
			<p class="text-muted-foreground mb-8">
				We are grateful to the following sponsors for their funding support:
			</p>

			<div class="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
				{#each [
					'Illinois Innovation Network',
					'NIH AIM-AHEAD',
					'NIU Office of Student Engagement and Experiential Learning',
					'NIU College of Health and Human Sciences',
					'SIUE Graduate School',
					'SIUE Undergraduate Research and Creative Activities'
				] as sponsor}
					<Card.Root class="bg-muted/50 hover:bg-muted transition-colors">
						<Card.Content class="flex items-center justify-center p-6 text-center">
							<span class="font-medium">{sponsor}</span>
						</Card.Content>
					</Card.Root>
				{/each}
			</div>
		</section>

		<!-- Special Thanks Section -->
		<section class="mt-20">
			<div class="flex items-center gap-3 mb-4">
				<div class="inline-flex h-10 w-10 items-center justify-center rounded-lg bg-primary/10">
					<Handshake class="h-5 w-5 text-primary" />
				</div>
				<h2 class="text-2xl font-bold">Special Thanks</h2>
			</div>
			<p class="text-muted-foreground mb-8">
				We appreciate the continued support by the following partners:
			</p>

			<div class="grid gap-3 sm:grid-cols-2">
				{#each [
					'NIU Center for Research Computing and Data',
					'NIU Center for the Interdisciplinary Study of Language and Literacy',
					'NIU Autism Caregiver Group',
					'NIU Speech-Language-Hearing Clinic',
					'NIU Division of Research and Innovation Partnerships',
					'NIU Foundation'
				] as partner}
					<div class="flex items-center gap-3 rounded-lg border p-4 hover:bg-muted/50 transition-colors">
						<div class="h-2 w-2 rounded-full bg-primary"></div>
						<span>{partner}</span>
					</div>
				{/each}
			</div>
		</section>
	</div>
</div>
