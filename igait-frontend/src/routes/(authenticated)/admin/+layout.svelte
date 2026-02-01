<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { getUser } from '$lib/hooks';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Shield, Layers, Users, AlertTriangle } from '@lucide/svelte';

	let { children } = $props();

	const user = getUser();

	// Redirect non-admins
	$effect(() => {
		if (!user.administrator) {
			goto('/dashboard');
		}
	});

	// Navigation items for admin panel
	const adminNavItems = [
		{ href: '/admin/queues', label: 'Queue Overview', icon: Layers },
		{ href: '/admin/jobs', label: 'Job Overview', icon: Users },
	];

	const currentPath = $derived($page.url.pathname);
</script>

{#if user.administrator}
	<div class="admin-panel">
		<div class="admin-header">
			<div class="admin-title">
				<Shield class="h-6 w-6 text-amber-500" />
				<h1>Admin Panel</h1>
				<Badge variant="outline" class="admin-badge">Administrator</Badge>
			</div>
			<p class="admin-description">
				Monitor system queues and manage all user submissions
			</p>
		</div>

		<nav class="admin-nav">
			{#each adminNavItems as item}
				<a
					href={item.href}
					class="admin-nav-item"
					class:active={currentPath === item.href}
				>
					<item.icon class="h-4 w-4" />
					{item.label}
				</a>
			{/each}
		</nav>

		<div class="admin-content">
			{@render children()}
		</div>
	</div>
{:else}
	<div class="access-denied">
		<AlertTriangle class="h-12 w-12 text-destructive" />
		<h2>Access Denied</h2>
		<p>You don't have permission to access this area.</p>
		<Button href="/dashboard">Return to Dashboard</Button>
	</div>
{/if}

<style>
	.admin-panel {
		display: flex;
		flex-direction: column;
		gap: var(--spacing-lg);
	}

	.admin-header {
		display: flex;
		flex-direction: column;
		gap: var(--spacing-xs);
	}

	.admin-title {
		display: flex;
		align-items: center;
		gap: var(--spacing-sm);
	}

	.admin-title h1 {
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: -0.025em;
	}

	:global(.admin-badge) {
		background-color: hsl(var(--amber-500) / 0.1) !important;
		border-color: hsl(var(--amber-500) / 0.3) !important;
		color: hsl(38 92% 50%) !important;
	}

	.admin-description {
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
	}

	.admin-nav {
		display: flex;
		gap: var(--spacing-sm);
		border-bottom: 1px solid hsl(var(--border));
		padding-bottom: var(--spacing-sm);
	}

	.admin-nav-item {
		display: flex;
		align-items: center;
		gap: var(--spacing-xs);
		padding: 0.5rem 1rem;
		border-radius: var(--radius-md);
		font-size: 0.875rem;
		font-weight: 500;
		color: hsl(var(--muted-foreground));
		transition: all 0.2s;
	}

	.admin-nav-item:hover {
		background-color: hsl(var(--muted));
		color: hsl(var(--foreground));
	}

	.admin-nav-item.active {
		background-color: hsl(var(--primary) / 0.1);
		color: hsl(var(--primary));
	}

	.admin-content {
		min-height: 400px;
	}

	.access-denied {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: var(--spacing-md);
		padding: 4rem 2rem;
		text-align: center;
	}

	.access-denied h2 {
		font-size: 1.5rem;
		font-weight: 600;
	}

	.access-denied p {
		color: hsl(var(--muted-foreground));
	}
</style>
