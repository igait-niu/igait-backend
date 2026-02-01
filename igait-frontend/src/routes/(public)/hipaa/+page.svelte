<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Accordion from '$lib/components/ui/accordion';
	import { Badge } from '$lib/components/ui/badge';
	import { Separator } from '$lib/components/ui/separator';
	import { Shield, Lock, FileText, AlertTriangle, Scale, Mail } from '@lucide/svelte';
</script>

<svelte:head>
	<title>HIPAA Compliance - iGait</title>
	<meta name="description" content="Learn about iGait's commitment to protecting your health information under HIPAA regulations." />
</svelte:head>

<div class="py-12">
	<div class="mx-auto max-w-4xl px-4">
		<!-- Header -->
		<div class="text-center mb-12">
			<Badge variant="secondary" class="mb-4">Legal</Badge>
			<h1 class="text-4xl font-bold tracking-tight">Notice of HIPAA Compliance</h1>
			<p class="mt-4 text-lg text-muted-foreground">
				Protecting Your Personal and Health Information
			</p>
		</div>

		<Card.Root>
			<Card.Header>
				<div class="flex items-center gap-3">
					<div class="inline-flex h-12 w-12 items-center justify-center rounded-lg bg-primary/10">
						<Shield class="h-6 w-6 text-primary" />
					</div>
					<div>
						<Card.Title class="text-2xl">Our Commitment to Privacy</Card.Title>
						<Card.Description>How we protect your protected health information (PHI)</Card.Description>
					</div>
				</div>
			</Card.Header>
			<Card.Content class="space-y-8">
				<!-- Introduction -->
				<div class="rounded-lg bg-muted/50 p-6">
					<p class="text-muted-foreground leading-relaxed">
						Northern Illinois University is committed to protecting the privacy of patient personal and health information. 
						Applicable Federal and State laws require us to maintain the privacy of your personal and health information. 
						This Notice explains our privacy practices, our legal duties, and your rights concerning your personal and health information.
					</p>
					<p class="mt-4 text-muted-foreground leading-relaxed">
						In this Notice, your personal or protected health information (PHI) is referred to as "health information" and includes 
						information regarding your health care and treatment with identifiable factors such as your name, age, address, income, 
						or other financial information.
					</p>
				</div>

				<Separator />

				<!-- How We Protect Your Information -->
				<section>
					<div class="flex items-center gap-2 mb-4">
						<Lock class="h-5 w-5 text-primary" />
						<h2 class="text-xl font-semibold">How We Protect Your Health Information</h2>
					</div>
					<ul class="space-y-3">
						{#each [
							'Treating all of your health information that we collect as confidential',
							'Stating confidentiality policies and practices in our policies, as well as disciplinary measures for privacy violations',
							'Restricting access to your health information only to those who need to know your health information',
							'Only disclosing your health information that is necessary and ensuring all those receiving information have agreed to protect and maintain confidentiality',
							'Maintaining physical, electronic, and procedural safeguards to comply with federal and state regulations'
						] as item}
							<li class="flex items-start gap-3">
								<div class="mt-1.5 h-2 w-2 rounded-full bg-primary flex-shrink-0"></div>
								<span class="text-muted-foreground">{item}</span>
							</li>
						{/each}
					</ul>
				</section>

				<Separator />

				<!-- Uses and Disclosures -->
				<section>
					<div class="flex items-center gap-2 mb-4">
						<FileText class="h-5 w-5 text-primary" />
						<h2 class="text-xl font-semibold">Uses and Disclosures</h2>
					</div>
					
					<Accordion.Root type="multiple" class="w-full">
						<Accordion.Item value="definitions">
							<Accordion.Trigger>Key Definitions</Accordion.Trigger>
							<Accordion.Content>
								<div class="space-y-4 pt-2">
									<div class="rounded-lg border p-4">
										<h4 class="font-semibold mb-2">Use</h4>
										<p class="text-sm text-muted-foreground">
											Applies only to activities within our organization such as sharing, employing, applying, 
											utilizing, examining, and analyzing information that identifies you.
										</p>
									</div>
									<div class="rounded-lg border p-4">
										<h4 class="font-semibold mb-2">Disclosure</h4>
										<p class="text-sm text-muted-foreground">
											Applies to activities outside of our organization, such as releasing, transferring, 
											or providing access to information about you to other parties.
										</p>
									</div>
								</div>
							</Accordion.Content>
						</Accordion.Item>

						<Accordion.Item value="authorization">
							<Accordion.Trigger>Uses Requiring Authorization</Accordion.Trigger>
							<Accordion.Content>
								<div class="space-y-4 pt-2 text-muted-foreground">
									<p>
										We may use or disclose PHI for purposes outside treatment, payment, or healthcare operations 
										when your appropriate authorization is obtained. An "authorization" is written permission beyond 
										the general consent that permits only specific disclosures.
									</p>
									<p>
										You may revoke all such authorizations at any time, provided each revocation is in writing. 
										You may not revoke an authorization to the extent that (1) we relied on that authorization; or 
										(2) if the authorization was obtained as a condition of obtaining insurance coverage, law provides 
										the insurer the right to contest the claim under the policy.
									</p>
								</div>
							</Accordion.Content>
						</Accordion.Item>
					</Accordion.Root>
				</section>

				<Separator />

				<!-- Limits of Confidentiality -->
				<section>
					<div class="flex items-center gap-2 mb-4">
						<AlertTriangle class="h-5 w-5 text-amber-500" />
						<h2 class="text-xl font-semibold">Limits of Confidentiality</h2>
					</div>
					<p class="text-muted-foreground mb-4">
						Personnel may be mandated reporters; therefore, confidentiality must be waived under the following conditions:
					</p>
					<div class="space-y-3">
						{#each [
							'If the staff believes that a client is at risk of harming himself/herself or anyone else',
							'If the staff believes that a child is at risk of being neglected or abused',
							'If the staff believes that an elder or disabled adult is being neglected, abused, or financially exploited',
							'If there is a court order for release of information',
							'If the client is a minor (under 12 years of age), the client does not have a legal right to keep therapy or assessment information confidential from his/her parent(s) or guardian(s)',
							'In accordance with Public Act 099-0278, if you are a current student enrolled at Northern Illinois University, we may contact your designated mental health emergency contact if you threaten to do harm to yourself or others'
						] as item}
							<div class="flex items-start gap-3 rounded-lg bg-amber-500/10 p-3">
								<AlertTriangle class="h-4 w-4 text-amber-500 mt-0.5 flex-shrink-0" />
								<span class="text-sm">{item}</span>
							</div>
						{/each}
					</div>
					<p class="mt-4 text-sm text-muted-foreground">
						Based on the IL Firearm Concealed Carry Law (FOID; Public Act 098-0063), clients who are determined to be a 
						clear and present danger to themselves and/or their community, or who are determined to be developmentally 
						or intellectually disabled, must be reported to the IL Department of Human Services FOID Mental Health 
						Reporting System within 24 hours of determination.
					</p>
				</section>

				<Separator />

				<!-- Your Rights -->
				<section>
					<div class="flex items-center gap-2 mb-4">
						<Scale class="h-5 w-5 text-primary" />
						<h2 class="text-xl font-semibold">Your Rights</h2>
					</div>
					
					<Accordion.Root type="multiple" class="w-full">
						<Accordion.Item value="restrictions">
							<Accordion.Trigger>Right to Request Restrictions</Accordion.Trigger>
							<Accordion.Content>
								<p class="text-muted-foreground pt-2">
									You have the right to request additional restrictions on certain uses and disclosures of protected health information. 
									We may not be able to accept your request, but if we do, we will uphold the restriction unless it is an emergency.
								</p>
							</Accordion.Content>
						</Accordion.Item>

						<Accordion.Item value="communications">
							<Accordion.Trigger>Right to Confidential Communications</Accordion.Trigger>
							<Accordion.Content>
								<p class="text-muted-foreground pt-2">
									You have the right to request and receive confidential communications of PHI by alternative means and at alternative locations.
								</p>
							</Accordion.Content>
						</Accordion.Item>

						<Accordion.Item value="inspect">
							<Accordion.Trigger>Right to Inspect and Copy</Accordion.Trigger>
							<Accordion.Content>
								<p class="text-muted-foreground pt-2">
									You have the right to inspect or obtain a copy (or both) of your health records. A reasonable fee may be charged for 
									copying or, if necessary, redacting the record. Access to your records may be limited or denied under certain circumstances, 
									but in most cases, you have a right to request a review of that decision.
								</p>
							</Accordion.Content>
						</Accordion.Item>

						<Accordion.Item value="amend">
							<Accordion.Trigger>Right to Amend</Accordion.Trigger>
							<Accordion.Content>
								<p class="text-muted-foreground pt-2">
									You have the right to request in writing an amendment of your health information for as long as PHI records are maintained. 
									The request must identify which information is incorrect and include an explanation of why you think it should be amended. 
									If the request is denied, a written explanation stating why will be provided to you. Amending a record does not mean that 
									any portion of your health information will be deleted.
								</p>
							</Accordion.Content>
						</Accordion.Item>

						<Accordion.Item value="accounting">
							<Accordion.Trigger>Right to an Accounting</Accordion.Trigger>
							<Accordion.Content>
								<p class="text-muted-foreground pt-2">
									You generally have the right to receive an accounting of disclosures of PHI. If your health information is disclosed 
									for any reason other than treatment, program evaluation, payment, or operation, you have the right to an accounting 
									of each disclosure for the previous six (6) years. If more than one (1) accounting is requested in a twelve (12) month 
									period, a reasonable fee may be charged.
								</p>
							</Accordion.Content>
						</Accordion.Item>

						<Accordion.Item value="paper">
							<Accordion.Trigger>Right to Paper Copy</Accordion.Trigger>
							<Accordion.Content>
								<p class="text-muted-foreground pt-2">
									If you receive this notice electronically (e.g., accessing a website), you have the right to obtain a paper copy of the notice upon request.
								</p>
							</Accordion.Content>
						</Accordion.Item>
					</Accordion.Root>
				</section>

				<Separator />

				<!-- Contact Information -->
				<section class="rounded-lg bg-muted/50 p-6">
					<div class="flex items-center gap-2 mb-4">
						<Mail class="h-5 w-5 text-primary" />
						<h2 class="text-xl font-semibold">Questions and Complaints</h2>
					</div>
					<p class="text-muted-foreground mb-4">
						For questions regarding this Notice or our privacy practices, please contact the University Privacy Officer. 
						If you are concerned that your privacy rights may have been violated, you may contact the University Privacy Officer 
						to make a complaint. You may also make a written complaint to the U.S. Department of Health and Human Services.
					</p>
					<p class="text-muted-foreground mb-6">
						<strong>If you choose to make a complaint, we will not retaliate in any way.</strong>
					</p>
					<Card.Root>
						<Card.Content class="pt-6">
							<h3 class="font-semibold mb-2">University Privacy Officer</h3>
							<address class="not-italic text-muted-foreground space-y-1">
								<p>Jack Yetter</p>
								<p>Health Services Building 217</p>
								<p>Northern Illinois University</p>
								<p>DeKalb, IL 60115</p>
								<a href="mailto:jyetter@niu.edu" class="text-primary hover:underline">jyetter@niu.edu</a>
							</address>
						</Card.Content>
					</Card.Root>
				</section>
			</Card.Content>
		</Card.Root>
	</div>
</div>
