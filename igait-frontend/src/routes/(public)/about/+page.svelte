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

<div class="about-page">
	<div class="page-container">
		<!-- Hero Header -->
		<div class="page-header">
			<Badge variant="secondary" class="header-badge">About Us</Badge>
			<h1 class="page-title">About iGait</h1>
			<p class="page-subtitle">
				Advancing early autism detection through innovative technology and collaborative research
			</p>
		</div>

		<!-- Mission Cards -->
		<section class="mission-section">
			<div class="mission-grid">
				<Card.Root class="mission-card">
					<Card.Header>
						<div class="mission-icon">
							<Sparkles class="h-6 w-6 text-primary" />
						</div>
						<Card.Title>Our Mission</Card.Title>
						<Card.Description class="mission-description">
							To make advanced gait analysis accessible to everyone through 
							cutting-edge AI technology and user-friendly interfaces.
						</Card.Description>
					</Card.Header>
				</Card.Root>

				<Card.Root class="mission-card">
					<Card.Header>
						<div class="mission-icon">
							<Users class="h-6 w-6 text-primary" />
						</div>
						<Card.Title>Our Team</Card.Title>
						<Card.Description class="mission-description">
							A dedicated team of researchers, engineers, and healthcare 
							professionals working across multiple Illinois universities.
						</Card.Description>
					</Card.Header>
				</Card.Root>

				<Card.Root class="mission-card">
					<Card.Header>
						<div class="mission-icon">
							<Heart class="h-6 w-6 text-primary" />
						</div>
						<Card.Title>Our Impact</Card.Title>
						<Card.Description class="mission-description">
							Helping identify gait patterns for early intervention and 
							improved outcomes for individuals and families.
						</Card.Description>
					</Card.Header>
				</Card.Root>
			</div>
		</section>

		<!-- Student Research Team Section -->
		<section class="content-section">
			<div class="section-header">
				<div class="section-icon">
					<GraduationCap class="h-5 w-5 text-primary" />
				</div>
				<h2 class="section-title">Student Research Team</h2>
			</div>

			<div class="team-grid">
				<!-- NIU -->
				<Card.Root>
					<Card.Header>
						<Card.Title class="university-title">
							<Building2 class="h-5 w-5 text-muted-foreground" />
							Northern Illinois University (NIU)
						</Card.Title>
					</Card.Header>
					<Card.Content class="team-content">
						<div>
							<h4 class="team-category">
								Graduate Students
							</h4>
							<div class="team-list">
								{#each ['Gabriela Ibarra', 'Mahesh Raju', 'Ricardo Torres', 'Alicia LaRouech', 'Viviana Cortes', 'Noelle Veome'] as name}
									<Badge variant="secondary">{name}</Badge>
								{/each}
							</div>
						</div>
						<Separator />
						<div>
							<h4 class="team-category">
								Undergraduate Students
							</h4>
							<div class="team-list">
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
						<Card.Title class="university-title">
							<Building2 class="h-5 w-5 text-muted-foreground" />
							Southern Illinois University Edwardsville (SIUE)
						</Card.Title>
					</Card.Header>
					<Card.Content class="team-content">
						<div>
							<h4 class="team-category">
								Graduate Students
							</h4>
							<div class="team-list">
								{#each ['Amit Shrestha', 'Prashanna Pandit'] as name}
									<Badge variant="secondary">{name}</Badge>
								{/each}
							</div>
						</div>
						<Separator />
						<div>
							<h4 class="team-category">
								Undergraduate Students
							</h4>
							<div class="team-list">
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
		<section class="content-section">
			<div class="section-header">
				<div class="section-icon">
					<Heart class="h-5 w-5 text-primary" />
				</div>
				<h2 class="section-title">Sponsors</h2>
			</div>
			<p class="section-intro">
				We are grateful to the following sponsors for their funding support:
			</p>

			<div class="sponsors-grid">
				{#each [
					'Illinois Innovation Network',
					'NIH AIM-AHEAD',
					'NIU Office of Student Engagement and Experiential Learning',
					'NIU College of Health and Human Sciences',
					'SIUE Graduate School',
					'SIUE Undergraduate Research and Creative Activities'
				] as sponsor}
					<Card.Root class="sponsor-card">
						<Card.Content class="sponsor-content">
							<span class="sponsor-name">{sponsor}</span>
						</Card.Content>
					</Card.Root>
				{/each}
			</div>
		</section>

		<!-- Special Thanks Section -->
		<section class="content-section">
			<div class="section-header">
				<div class="section-icon">
					<Handshake class="h-5 w-5 text-primary" />
				</div>
				<h2 class="section-title">Special Thanks</h2>
			</div>
			<p class="section-intro">
				We appreciate the continued support by the following partners:
			</p>

			<div class="partners-grid">
				{#each [
					'NIU Center for Research Computing and Data',
					'NIU Center for the Interdisciplinary Study of Language and Literacy',
					'NIU Autism Caregiver Group',
					'NIU Speech-Language-Hearing Clinic',
					'NIU Division of Research and Innovation Partnerships',
					'NIU Foundation'
				] as partner}
					<div class="partner-item">
						<div class="partner-dot"></div>
						<span>{partner}</span>
					</div>
				{/each}
			</div>
		</section>
	</div>
</div>

<style>
	.about-page {
		padding-top: var(--spacing-lg);
		padding-bottom: var(--spacing-lg);
	}

	/* Header */
	.page-header {
		text-align: center;
	}

	.header-badge {
		margin-bottom: 1rem;
	}

	.page-title {
		font-size: 2.25rem;
		font-weight: 700;
		line-height: 1.1;
		letter-spacing: -0.025em;
	}

	@media (min-width: 640px) {
		.page-title {
			font-size: 3rem;
		}
	}

	.page-subtitle {
		margin-left: auto;
		margin-right: auto;
		margin-top: 1rem;
		max-width: var(--container-medium);
		font-size: 1.125rem;
		color: hsl(var(--muted-foreground));
	}

	/* Mission Section */
	.mission-section {
		margin-top: 4rem;
	}

	.mission-grid {
		display: grid;
		gap: 1.5rem;
		grid-template-columns: 1fr;
	}

	@media (min-width: 768px) {
		.mission-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.mission-card {
		border-width: 2px;
		transition: border-color 0.2s;
	}

	.mission-card:hover {
		border-color: hsl(var(--primary) / 0.5);
	}

	.mission-icon {
		margin-bottom: 1rem;
		display: inline-flex;
		height: 3rem;
		width: 3rem;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-lg);
		background-color: hsl(var(--primary) / 0.1);
	}

	.mission-description {
		font-size: 1rem;
	}

	/* Content Sections */
	.content-section {
		margin-top: 5rem;
	}

	.section-header {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 2rem;
	}

	.section-icon {
		display: inline-flex;
		height: 2.5rem;
		width: 2.5rem;
		align-items: center;
		justify-content: center;
		border-radius: var(--radius-lg);
		background-color: hsl(var(--primary) / 0.1);
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 700;
	}

	.section-intro {
		color: hsl(var(--muted-foreground));
		margin-bottom: 2rem;
	}

	/* Team Section */
	.team-grid {
		display: grid;
		gap: var(--grid-gap);
		grid-template-columns: 1fr;
	}

	@media (min-width: 1024px) {
		.team-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.university-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.team-content {
		display: flex;
		flex-direction: column;
		gap: 1.5rem;
	}

	.team-category {
		font-weight: 600;
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
		text-transform: uppercase;
		letter-spacing: 0.05em;
		margin-bottom: 0.75rem;
	}

	.team-list {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	/* Sponsors Section */
	.sponsors-grid {
		display: grid;
		gap: 1rem;
		grid-template-columns: 1fr;
	}

	@media (min-width: 640px) {
		.sponsors-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (min-width: 1024px) {
		.sponsors-grid {
			grid-template-columns: repeat(3, 1fr);
		}
	}

	.sponsor-card {
		background-color: hsl(var(--muted) / 0.5);
		transition: background-color 0.2s;
	}

	.sponsor-card:hover {
		background-color: hsl(var(--muted));
	}

	.sponsor-content {
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 1.5rem;
		text-align: center;
	}

	.sponsor-name {
		font-weight: 500;
	}

	/* Partners Section */
	.partners-grid {
		display: grid;
		gap: 0.75rem;
		grid-template-columns: 1fr;
	}

	@media (min-width: 640px) {
		.partners-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	.partner-item {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		border-radius: var(--radius-lg);
		border: 1px solid hsl(var(--border));
		padding: 1rem;
		transition: background-color 0.2s;
	}

	.partner-item:hover {
		background-color: hsl(var(--muted) / 0.5);
	}

	.partner-dot {
		height: 0.5rem;
		width: 0.5rem;
		border-radius: 9999px;
		background-color: hsl(var(--primary));
		flex-shrink: 0;
	}
</style>
