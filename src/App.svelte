<script lang="ts">
	import Titlebar from "$lib/components/titlebar.svelte";
	import { navigation, Page } from "$lib/navigation.svelte";
	import { Activity, Home, NotFound } from "$lib/pages";
	import { Toaster } from "$lib/components/ui/sonner";
</script>

<div class="h-screen select-none flex flex-col overflow-hidden">
	<Titlebar />

	{#if navigation.currentPage === Page.HOME}
		<Home />
	{:else if navigation.currentPage === Page.ACTIVITIES}
		<Activity />
	{:else}
		<NotFound />
	{/if}

	<Toaster
		closeButton={true}
		toastOptions={{
			duration: 2500,
			classes: {
				title: "text-base font-bold text-popover-foreground!",
				description: "text-muted-foreground!",
				toast: "gap-3!",
				error: "text-destructive!",
				success: "text-success!",
			}
		}}
	/>v
</div>
