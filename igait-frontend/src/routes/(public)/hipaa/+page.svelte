<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Shield } from '@lucide/svelte';
</script>

<svelte:head>
	<title>HIPAA Compliance - iGait</title>
</svelte:head>

<div class="py-12">
	<div class="mx-auto max-w-4xl px-4">
		<Card.Root>
			<Card.Header>
				<div class="mb-4 inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
					<Shield class="h-6 w-6 text-primary" />
				</div>
				<Card.Title class="text-3xl">HIPAA Compliance</Card.Title>
				<Card.Description>
					Our commitment to protecting your health information
				</Card.Description>
			</Card.Header>
			<Card.Content class="prose prose-slate max-w-none dark:prose-invert">
				<h2>Our Commitment to Privacy</h2>
				<p>
					iGait is committed to maintaining the privacy and security of your health 
					information in accordance with the Health Insurance Portability and 
					Accountability Act (HIPAA).
				</p>

				<h2>Protected Health Information (PHI)</h2>
				<p>
					We implement administrative, physical, and technical safeguards to protect 
					any health information you provide to us. This includes:
				</p>
				<ul>
					<li>Encryption of data in transit and at rest</li>
					<li>Access controls and authentication</li>
					<li>Audit logging and monitoring</li>
					<li>Regular security assessments</li>
				</ul>

				<h2>Your Rights Under HIPAA</h2>
				<p>
					Under HIPAA, you have the right to:
				</p>
				<ul>
					<li>Access your health information</li>
					<li>Request amendments to your records</li>
					<li>Receive an accounting of disclosures</li>
					<li>Request restrictions on uses and disclosures</li>
					<li>File a complaint if you believe your rights have been violated</li>
				</ul>

				<h2>Research Use</h2>
				<p>
					If your data is used for research purposes, it will be de-identified in 
					accordance with HIPAA requirements, or you will be asked to provide 
					explicit authorization.
				</p>

				<h2>Contact Our Privacy Officer</h2>
				<p>
					For questions about our HIPAA compliance or to exercise your rights, 
					please contact our Privacy Officer at hipaa@igaitapp.com.
				</p>
			</Card.Content>
		</Card.Root>
	</div>
</div>
