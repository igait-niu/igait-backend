<script lang="ts">
	import { getUser } from '$lib/hooks';
	import * as Card from '$lib/components/ui/card';
	import { Button } from '$lib/components/ui/button';
	import { Construction, MessageSquare } from '@lucide/svelte';

	const user = getUser();
</script>

<svelte:head>
	<title>AI Assistant - iGait</title>
</svelte:head>

<div class="space-y-8">
	<section>
		<h1 class="text-3xl font-bold tracking-tight">AI Assistant</h1>
		<p class="mt-2 text-muted-foreground">
			Chat with our AI assistant about your gait analysis results
		</p>
	</section>

	<!-- Coming Soon Placeholder -->
	<Card.Root class="text-center">
		<Card.Header>
			<div class="mx-auto mb-4 flex h-16 w-16 items-center justify-center rounded-full bg-primary/10">
				<Construction class="h-8 w-8 text-primary" />
			</div>
			<Card.Title class="text-2xl">Coming Soon!</Card.Title>
			<Card.Description class="text-base">
				The AI Assistant feature is currently being built with love~ 🦊
			</Card.Description>
		</Card.Header>
		<Card.Content>
			<p class="mx-auto max-w-md text-sm text-muted-foreground">
				Soon you'll be able to chat with our AI assistant to ask questions about 
				your gait analysis results, get personalized insights, and learn more about 
				the iGait system.
			</p>
			<div class="mt-6 flex items-center justify-center gap-4">
				<Button variant="outline" href="/dashboard">
					Back to Dashboard
				</Button>
				<Button href="/submit">
					<MessageSquare class="mr-2 h-4 w-4" />
					Make a Submission
				</Button>
			</div>
		</Card.Content>
	</Card.Root>
</div>
