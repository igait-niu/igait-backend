<script lang="ts">
	import * as Card from '$lib/components/ui/card';
</script>

<svelte:head>
	<title>Privacy Policy - iGait</title>
</svelte:head>

<div class="py-12">
	<div class="mx-auto max-w-4xl px-4">
		<Card.Root>
			<Card.Header>
				<Card.Title class="text-3xl">Privacy Policy</Card.Title>
				<Card.Description>Last updated: February 1, 2026</Card.Description>
			</Card.Header>
			<Card.Content class="prose prose-slate max-w-none dark:prose-invert">
				<h2>1. Information We Collect</h2>
				<p>
					We collect information you provide directly to us, including:
				</p>
				<ul>
					<li>Account information (email, name)</li>
					<li>Video recordings for gait analysis</li>
					<li>Usage data and analytics</li>
				</ul>

				<h2>2. How We Use Your Information</h2>
				<p>
					We use the information we collect to:
				</p>
				<ul>
					<li>Provide and maintain our services</li>
					<li>Process and analyze gait data</li>
					<li>Send you results and notifications</li>
					<li>Improve our algorithms and services</li>
				</ul>

				<h2>3. Data Storage and Security</h2>
				<p>
					Your data is stored securely using industry-standard encryption. Video 
					files are processed and then stored securely. We implement appropriate 
					technical and organizational measures to protect your personal data.
				</p>

				<h2>4. Data Sharing</h2>
				<p>
					We do not sell your personal information. We may share anonymized, 
					aggregated data for research purposes. Your video data is never shared 
					with third parties without your explicit consent.
				</p>

				<h2>5. Your Rights</h2>
				<p>
					You have the right to:
				</p>
				<ul>
					<li>Access your personal data</li>
					<li>Request correction of your data</li>
					<li>Request deletion of your data</li>
					<li>Withdraw consent at any time</li>
				</ul>

				<h2>6. Data Retention</h2>
				<p>
					We retain your data for as long as your account is active or as needed 
					to provide you services. You may request deletion of your data at any time.
				</p>

				<h2>7. Contact Us</h2>
				<p>
					If you have questions about this Privacy Policy, please contact us at 
					privacy@igaitapp.com.
				</p>
			</Card.Content>
		</Card.Root>
	</div>
</div>
