<script lang="ts">
	import { getUser } from '$lib/hooks';
	import QuickActionsSection from './QuickActionsSection.svelte';
	import StatsSection from './StatsSection.svelte';
	import RecentActivitySection from './RecentActivitySection.svelte';

	// Get the authenticated user - guaranteed to exist in (authenticated) routes
	const user = getUser();
</script>

<svelte:head>
	<title>Dashboard - iGait</title>
</svelte:head>

<div class="stack-lg">
	<!-- Welcome Section -->
	<section>
		<h1 class="dashboard-title">
			Welcome back, {user.displayName.split(' ')[0]}!
		</h1>
		<p class="dashboard-subtitle">
			Here's an overview of your gait analysis activity.
		</p>
	</section>

	<QuickActionsSection />
	<StatsSection />
	<RecentActivitySection />
</div>

<style>
	.dashboard-title {
		font-size: 1.5rem;
		font-weight: 700;
	}

	@media (min-width: 640px) {
		.dashboard-title {
			font-size: 1.875rem;
		}
	}

	.dashboard-subtitle {
		margin-top: var(--spacing-xs);
		color: hsl(var(--muted-foreground));
	}
</style>