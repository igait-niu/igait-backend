<script lang="ts">
	import { Separator } from '$lib/components/ui/separator';
</script>

<!--
	Shared footer component with legal links
	Used across all layouts to ensure legal pages are always accessible
-->

<footer class="border-t bg-background">
	<div class="mx-auto max-w-7xl px-4 py-8">
		<div class="flex flex-col gap-6">
			<!-- Main footer content -->
			<div class="flex flex-col items-center justify-between gap-4 sm:flex-row">
                <p class="text-xs text-muted-foreground">
					© {new Date().getFullYear()} iGait · Northern Illinois University
				</p>

				<!-- Navigation links -->
				<nav class="flex flex-wrap items-center justify-center gap-x-6 gap-y-2">
					<a 
						href="/about" 
						class="text-sm text-muted-foreground transition-colors hover:text-foreground"
					>
						About
					</a>
					<a 
						href="/terms" 
						class="text-sm text-muted-foreground transition-colors hover:text-foreground"
					>
						Terms of Service
					</a>
					<a 
						href="/policy" 
						class="text-sm text-muted-foreground transition-colors hover:text-foreground"
					>
						Privacy Policy
					</a>
					<a 
						href="/hipaa" 
						class="text-sm text-muted-foreground transition-colors hover:text-foreground"
					>
						HIPAA Compliance
					</a>
				</nav>
			</div>
		</div>
	</div>
</footer>
