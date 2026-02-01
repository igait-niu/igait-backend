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

<div class="space-y-8">
	<section>
		<h1 class="text-3xl font-bold tracking-tight">Settings</h1>
		<p class="mt-2 text-muted-foreground">
			Manage your account preferences
		</p>
	</section>

	<!-- Coming Soon Placeholder -->
	<Card.Root class="text-center">
		<Card.Header>
			<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-primary/10">
				<Construction class="h-8 w-8 text-primary" />
			</div>
			<Card.Title class="text-2xl">Settings Coming Soon!</Card.Title>
			<Card.Description class="text-base">
				Account settings are being implemented~ 🦊
			</Card.Description>
		</Card.Header>
		<Card.Content>
			<p class="mx-auto max-w-md text-sm text-muted-foreground">
				Soon you'll be able to update your profile, manage notification preferences, 
				and configure your account settings here.
			</p>
			
			<div class="mt-6 rounded-lg bg-muted p-4">
				<h3 class="font-semibold">Your Current Account</h3>
				<p class="text-sm text-muted-foreground">{user.email}</p>
				<p class="text-sm text-muted-foreground">{user.displayName}</p>
			</div>

			<Button variant="outline" href="/dashboard" class="mt-6">
				Back to Dashboard
			</Button>
		</Card.Content>
	</Card.Root>
</div>
