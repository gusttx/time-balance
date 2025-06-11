<script lang="ts">
	import { X, Minus, Square, PinOff, Pin } from "@lucide/svelte";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu/";
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { navigation, Page, pages } from "$lib/navigation.svelte";
	import { cn, isDesktop } from "$lib/utils";

	const window = getCurrentWindow();

	let isOpen = $state(false);
	let alwaysOnTop = $state(false);

	window.isAlwaysOnTop().then((value) => (alwaysOnTop = value));

	const currentPage = $derived(pages.get(navigation.currentPage));
	const PinIcon = $derived(alwaysOnTop ? PinOff : Pin);

	function navigate(page: Page) {
		isOpen = false;
		navigation.navigate(page);
	}

	function toggleAlwaysOnTop() {
		const window = getCurrentWindow();
		alwaysOnTop = !alwaysOnTop;
		window.setAlwaysOnTop(alwaysOnTop);
	}
</script>

<nav
	data-tauri-drag-region
	class="text-ring border-b-border relative flex w-full items-center border-b-[1px] p-1 text-sm"
>
	<DropdownMenu.Root bind:open={isOpen}>
		<DropdownMenu.Trigger
			class={cn(buttonVariants({ variant: "ghost", size: "sm" }), "text-muted-foreground")}
		>
			<currentPage.icon />
			{currentPage.name}
		</DropdownMenu.Trigger>
		<DropdownMenu.Content align="start">
			{#each pages.all() as page}
				<DropdownMenu.Item
					class="cursor-pointer"
					disabled={page.page === currentPage.page}
					onclick={() => navigate(page.page)}
				>
					<page.icon />
					{page.name}
				</DropdownMenu.Item>
			{/each}
		</DropdownMenu.Content>
	</DropdownMenu.Root>
	{#if isDesktop}
		<div class="absolute right-1 flex gap-1">
			<Button variant="ghost" size="icon" onclick={toggleAlwaysOnTop}>
				<PinIcon />
			</Button>
			<Button variant="ghost" size="icon" class="cursor-default" onclick={() => window.minimize()}>
				<Minus />
			</Button>
			<Button variant="ghost" size="icon" disabled>
				<Square />
			</Button>
			<Button
				variant="ghost"
				size="icon"
				class="hover:text-foreground cursor-default hover:bg-red-600 dark:hover:bg-red-600"
				onclick={() => window.close()}
			>
				<X />
			</Button>
		</div>
	{/if}
</nav>
