<script lang="ts">
	import Titlebar from "$lib/components/titlebar.svelte";
	import { navigation, Page } from "$lib/navigation.svelte";
	import { Toaster } from "$lib/components/ui/sonner";
	import Entry from "$lib/pages/entry.svelte";
	import Activity from "$lib/pages/activity.svelte";
	import Home from "$lib/pages/home.svelte";
	import NotFound from "$lib/pages/not-found.svelte";
	import type { Component } from "svelte";
	import { TooltipProvider } from "$lib/components/ui/tooltip";

	const pages = new Map([
		[Page.HOME, Home],
		[Page.ACTIVITIES, Activity],
		[Page.ENTRIES, Entry],
	]);

	const CurrentPage: Component = $derived(pages.get(navigation.currentPage) ?? NotFound);
</script>

<div class="h-screen select-none flex flex-col overflow-hidden">
	<TooltipProvider>
		<Titlebar />
	
		<CurrentPage />
	
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
					info: "text-blue-500!"
				}
			}}
		/>
	</TooltipProvider>
</div>
