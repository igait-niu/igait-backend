<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import MissionSection from './MissionSection.svelte';
	import TeamSection from './TeamSection.svelte';
	import SponsorsSection from './SponsorsSection.svelte';
</script>

<svelte:head>
	<title>About - iGait</title>
	<meta name="description" content="Learn about the iGait research team advancing early autism detection through innovative gait analysis technology." />
</svelte:head>

<div class="about-page">
	<div class="page-container">
		<div class="page-header">
			<Badge variant="secondary" class="header-badge">About Us</Badge>
			<h1 class="page-title">About iGait</h1>
			<p class="page-subtitle">
				Advancing early autism detection through innovative technology and collaborative research
			</p>
		</div>

		<MissionSection />
		<TeamSection />
		<SponsorsSection />
	</div>
</div>

<style>
	.about-page {
		padding-top: var(--spacing-lg);
		padding-bottom: var(--spacing-lg);
	}

	.page-header {
		text-align: center;
	}

	:global(.header-badge) {
		margin-bottom: 1rem;
	}

	.page-title {
		font-size: 1.875rem;
		font-weight: 700;
		line-height: 1.1;
		letter-spacing: -0.025em;
	}

	@media (min-width: 640px) {
		.page-title {
			font-size: 2.25rem;
		}
	}

	@media (min-width: 768px) {
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
</style>
