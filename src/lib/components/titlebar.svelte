<script lang="ts">
	import { X, Minus, Square, Dumbbell, House, FileQuestion, type IconProps, PinOff, Pin } from "@lucide/svelte";
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu/";
	import { Button, buttonVariants } from "$lib/components/ui/button";
	import { getCurrentWindow } from "@tauri-apps/api/window";
	import { navigation, Page } from "$lib/navigation.svelte";
	import { cn, isDesktop } from "$lib/utils";
	import type { Component } from "svelte";

	interface PageInfo {
		page: Page;
		name: string;
		icon: Component<IconProps>;
	}

	const notFoundPage: PageInfo = {
		page: Page.NOT_FOUND,
		name: "Unknown",
		icon: FileQuestion
	};

	const pages: PageInfo[] = [
		{
			page: Page.HOME,
			name: "Home",
			icon: House
		},
		{
			page: Page.ACTIVITIES,
			name: "Activities",
			icon: Dumbbell
		}
	];

	const window = getCurrentWindow();

	let isOpen = $state(false);
	let alwaysOnTop = $state(false);

	window.isAlwaysOnTop().then((value) => (alwaysOnTop = value));

	const currentPage = $derived(
		pages.find(({ page }) => page === navigation.currentPage) ?? notFoundPage
	);

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
			{#each pages as page}
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
		<ul class="absolute right-1 flex gap-1">
			<li>
				<Button
					variant="ghost"
					size="icon"
					onclick={toggleAlwaysOnTop}
				>
					{#if alwaysOnTop}
						<PinOff />
					{:else}
						<Pin />
					{/if}
				</Button>
			</li>
			<li>
				<Button
					variant="ghost"
					size="icon"
					class="cursor-default"
					onclick={() => window.minimize()}
				>
					<Minus />
				</Button>
			</li>
			<li>
				<Button variant="ghost" size="icon" disabled>
					<Square />
				</Button>
			</li>
			<li>
				<Button
					variant="ghost"
					size="icon"
					class="hover:text-foreground cursor-default hover:bg-red-600 dark:hover:bg-red-600"
					onclick={() => window.close()}
				>
					<X />
				</Button>
			</li>
		</ul>
	{/if}
</nav>
