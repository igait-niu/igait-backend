<script lang="ts">
	import { getUser } from '$lib/hooks';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Construction, Settings } from '@lucide/svelte';

	const user = getUser();
</script>

<svelte:head>
	<title>Settings - iGait</title>
</svelte:head>

<div class="settings-page stack-lg">
	<section>
		<h1 class="page-title">Settings</h1>
		<p class="page-description">
			Manage your account preferences
		</p>
	</section>

	<!-- Coming Soon Placeholder -->
	<Card.Root class="coming-soon-card">
		<Card.Header>
			<div class="coming-soon-icon">
				<Construction class="h-8 w-8 text-primary" />
			</div>
			<Card.Title class="coming-soon-title">Settings Coming Soon!</Card.Title>
			<Card.Description class="coming-soon-description">
				Account settings are being implemented~ 🦊
			</Card.Description>
		</Card.Header>
		<Card.Content>
			<p class="coming-soon-text">
				Soon you'll be able to update your profile, manage notification preferences, 
				and configure your account settings here.
			</p>
			
			<div class="account-info">
				<h3 class="account-title">Your Current Account</h3>
				<p class="account-detail">{user.email}</p>
				<p class="account-detail">{user.displayName}</p>
			</div>

			<Button variant="outline" href="/dashboard" class="back-button">
				Back to Dashboard
			</Button>
		</Card.Content>
	</Card.Root>
</div>

<style>
	.page-title {
		font-size: 1.5rem;
		font-weight: 700;
		line-height: 1.2;
		letter-spacing: -0.025em;
	}

	@media (min-width: 640px) {
		.page-title {
			font-size: 1.875rem;
		}
	}

	.page-description {
		margin-top: 0.5rem;
		color: hsl(var(--muted-foreground));
	}

	:global(.coming-soon-card) {
		text-align: center;
	}

	.coming-soon-icon {
		margin: 0 auto 1rem;
		display: flex;
		height: 4rem;
		width: 4rem;
		align-items: center;
		justify-content: center;
		border-radius: 9999px;
		background-color: hsl(var(--primary) / 0.1);
		animation: pulse 2s ease-in-out infinite;
	}

	@keyframes pulse {
		0%, 100% {
			opacity: 1;
		}
		50% {
			opacity: 0.7;
		}
	}

	:global(.coming-soon-title) {
		font-size: 1.5rem;
	}

	:global(.coming-soon-description) {
		font-size: 1rem;
	}

	.coming-soon-text {
		margin: 0 auto;
		max-width: 28rem;
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
	}

	.account-info {
		margin-top: 1.5rem;
		border-radius: var(--radius-lg);
		background-color: hsl(var(--muted));
		padding: 1rem;
	}

	.account-title {
		font-weight: 600;
	}

	.account-detail {
		font-size: 0.875rem;
		color: hsl(var(--muted-foreground));
	}

	:global(.back-button) {
		margin-top: 1.5rem;
	}
</style>
