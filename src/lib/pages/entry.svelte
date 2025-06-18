<script lang="ts" module>
	import Stopwatch from "$lib/components/stopwatch.svelte";
	import { Page, pages } from "$lib/navigation.svelte";
	import PageTemplate from "./page-template.svelte";
	import { entries } from "$lib/entries.svelte";
	import * as Pagination from "$lib/components/ui/pagination/index.js";
	import { ChevronLeftIcon, ChevronRightIcon, CircleDotDashed, Sparkles, Trash2 } from "@lucide/svelte";
	import { cn } from "$lib/utils";
	import Button from "$lib/components/ui/button/button.svelte";
	import { deleteEntry, type Entry } from "$lib/commands/entry";
	import { toast } from "svelte-sonner";

	function formatDuration(secs: number) {
		const h = Math.floor(secs / 3600).toString();
		const m = Math.floor((secs % 3600) / 60).toString();
		const s = (secs % 60).toString();

		return `${h.padStart(2, "0")}:${m.padStart(2, "0")}:${s.padStart(2, "0")}`;
	}

	const numberFormater = new Intl.NumberFormat("en-US", {
		style: "decimal",
	})
</script>

<script lang="ts">
	const thisPage = pages.get(Page.ENTRIES);

	let deleting = $state(false);

	function delEntry(entry: Entry) {
		return async () => {
			if (deleting) return;

			deleting = true;
			const result = await deleteEntry(entry.id);

			if (!result.ok) {
				deleting = false;
				toast.error(`${result.error.error} error`, {
					description: result.error.message
				});
				return;
			}

			await entries.loadPage(entries.page);
			deleting = false;

			toast.success("Entry deleted", {
				description: "The entry has been deleted successfully."
			});
		}
	}
</script>

<PageTemplate
	header={{
		icon: thisPage.icon,
		title: thisPage.name,
		description: "Here you can list and manage your entries.",
		class: "bg-linear-[155deg] from-entry2/40 to-entry/25"
	}}
	class="flex flex-col gap-3"
>
	<Stopwatch />

	{#if entries.total > 10}
		<Pagination.Root count={entries.total} perPage={10} bind:page={entries.page}>
			{#snippet children({ pages, currentPage })}
				<Pagination.Content>
					<Pagination.Item>
						<Pagination.PrevButton>
							<ChevronLeftIcon />
						</Pagination.PrevButton>
					</Pagination.Item>

					{#each pages as page (page.key)}
						{#if page.type === "ellipsis"}
							<Pagination.Item>
								<Pagination.Ellipsis />
							</Pagination.Item>
						{:else}
							<Pagination.Item>
								<Pagination.Link {page} isActive={page.value === currentPage}>
									{page.value}
								</Pagination.Link>
							</Pagination.Item>
						{/if}
					{/each}
					<Pagination.Item>
						<Pagination.NextButton>
							<ChevronRightIcon />
						</Pagination.NextButton>
					</Pagination.Item>
				</Pagination.Content>
			{/snippet}
		</Pagination.Root>
	{/if}

	<div class="gap-3 grid grid-cols-1 sm:grid-cols-2 place-items-center">
		{#each entries.entries as entry (entry.id)}
			{@const points = entry.points_per_second * entry.duration}
			<div class="bg-linear-[135deg] from-input/5 via-input/25 border-border border-1 rounded-md w-full p-2 flex flex-col gap-1 text-sm backdrop-blur-xs max-w-[280px]">
				<div class="flex items-center gap-1 overflow-hidden">
					<CircleDotDashed size={16}/>
					<p class="text-center truncate">{entry.activity_name}</p>
				</div>

				<div class="flex items-center gap-1 justify-between font-dosis">
					<div class="text-muted-foreground">
						{formatDuration(entry.duration)}
					</div>
					<div class={cn("flex items-center gap-1 text-muted-foreground drop-shadow-[0_0_3px_var(--muted-foreground)]", {
						"text-destructive drop-shadow-[0_0_3px_var(--destructive)]": points < 0,
						"text-green-400 drop-shadow-[0_0_3px_var(--color-green-400)]": points > 0
					})}>
						{#if points > 0}
							+{numberFormater.format(points)}
						{:else}
							{numberFormater.format(points)}
						{/if}
						<Sparkles size={16} />
					</div>
				</div>

				<div class="flex justify-end items-center">
					<Button variant="link" size="sm" class="p-0 text-destructive h-auto font-normal" onclick={delEntry(entry)}>
						delete
					</Button>
				</div>
			</div>
		{/each}
	</div>
</PageTemplate>
