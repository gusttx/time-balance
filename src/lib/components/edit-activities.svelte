<script lang="ts">
	import Button, { buttonVariants } from "$lib/components/ui/button/button.svelte";
	import * as Drawer from "$lib/components/ui/drawer";
	import Input from "$lib/components/ui/input/input.svelte";
	import Separator from "$lib/components/ui/separator/separator.svelte";
	import { CircleDotDashed, RotateCw, Sparkles } from "@lucide/svelte";
	import Slider from "./ui/slider/slider.svelte";
	import { editActivity, deleteActivity, type Activity } from "$lib/commands/activity";
	import { activities } from "$lib/activities.svelte";
	import { toast } from "svelte-sonner";
	import { cn } from "$lib/utils";
	import { tick } from "svelte";

	let open = $state(false);
	let name = $state("");
	let points = $state(0);
	let id = $state(0);
	let invalidName = $state(false);

	let input = $state<HTMLInputElement | null>(null);

	function setActivity(activity: Activity) {
		return () => {
			name = activity.name;
			points = activity.points_per_second;
			id = activity.id;

			tick().then(() => input?.focus());
			open = true;
		};
	}

	function delActivity() {
		deleteActivity(id).then((result) => {
			if (!result.ok) {
				toast.error(`${result.error.error} error`, {
					description: result.error.message
				});
				return;
			}

			activities.remove(id);

			toast.success("Activity deleted", {
				description: "The activity has been deleted successfully."
			});
		});

		open = false;
	}

	function saveActivity() {
		if (!name.trim()) {
			invalidName = true;
			return;
		}

		editActivity(id, name, points).then((result) => {
			if (!result.ok) {
				toast.error(`${result.error.error} error`, {
					description: result.error.message
				});
				return;
			}

			activities.edit(result.value);

			toast.success("Activity edited", {
				description: "The activity has been edited successfully."
			});
		});

		open = false;
	}
</script>

{#if activities.error}
	<div class="mt-4 flex flex-col items-center gap-2">
		<p class="text-destructive text-sm">Error loading activities</p>
		<Button variant="outline" onclick={() => activities.load()}>
			<RotateCw /> Try again
		</Button>
	</div>
{:else}
	<Drawer.Root bind:open>
		<ol class="mt-4 flex w-full flex-col gap-2">
			{#each activities.entries as activity (activity.id)}
				<li>
					<Button variant="outline" class="w-full" onclick={setActivity(activity)}>
						<CircleDotDashed />
						<div class="flex grow items-center justify-between truncate">
							<div class="truncate">
								{activity.name}
							</div>
							<div
								class={cn(
									"font-dosis text-muted-foreground flex items-center justify-center gap-1",
									{
										"text-destructive": activity.points_per_second < 0,
										"text-green-400": activity.points_per_second > 0
									}
								)}
							>
								{#if activity.points_per_second > 0}
									+{activity.points_per_second}
								{:else}
									{activity.points_per_second}
								{/if}
								<Sparkles size={16} />
							</div>
						</div>
					</Button>
				</li>
			{/each}
		</ol>
		<Drawer.Content>
			<Drawer.Header>
				<Drawer.Title>Edit activity</Drawer.Title>
				<Drawer.Description>You can edit or delete this activity.</Drawer.Description>
			</Drawer.Header>

			<Separator />

			<div class="mb-4 flex flex-col gap-4">
				<div class="flex flex-col gap-1">
					<label for="name" class="text-base font-medium">Activity name</label>
					<Input
						id="name"
						placeholder="Learn Rust"
						maxlength={30}
						minlength={1}
						bind:value={name}
						bind:ref={input}
					/>
					{#if invalidName}
						<p class="text-destructive text-sm">Activity name is required</p>
					{/if}
				</div>

				<div class="flex flex-col gap-2">
					<label for="points" class="text-base font-medium">
						Points per second
						<span class="text-muted-foreground font-thin">
							({points})
						</span>
					</label>
					<Slider type="single" bind:value={points} min={-10} max={10} data-vaul-no-drag />
				</div>
			</div>

			<Drawer.Footer class="flex-row justify-between gap-2">
				<Button
					variant="ghost"
					class="text-destructive hover:text-destructive/80"
					onclick={delActivity}>Delete</Button
				>
				<div class="flex gap-2">
					<Drawer.Close class={buttonVariants({ variant: "ghost" })}>Cancel</Drawer.Close>
					<Button class="ring-foreground bg-green-400 hover:bg-green-300" onclick={saveActivity}
						>Salvar</Button
					>
				</div>
			</Drawer.Footer>
		</Drawer.Content>
	</Drawer.Root>
{/if}
