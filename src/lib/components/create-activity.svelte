<script lang="ts">
	import Button, { buttonVariants } from "$lib/components/ui/button/button.svelte";
	import * as Drawer from "$lib/components/ui/drawer";
	import Input from "$lib/components/ui/input/input.svelte";
	import Separator from "$lib/components/ui/separator/separator.svelte";
	import { Plus } from "@lucide/svelte";
	import Slider from "./ui/slider/slider.svelte";
	import { createActivity } from "$lib/commands/activity";
	import { activities } from "$lib/activities.svelte";
	import { toast } from "svelte-sonner";
	import { cn } from "$lib/utils";

	let open = $state(false);
	let name = $state("");
	let points = $state(0);
	let invalidName = $state(false);

	function clear() {
		name = "";
		points = 0;
		invalidName = false;
	}

	function handleSubmit() {
		if (!name.trim()) {
			invalidName = true;
			return;
		}

		createActivity(name, points).then((result) => {
			if (!result.ok) {
				const { error, message } = result.error;
				toast.error(`${error} error`, {
					description: message
				});
				return;
			}

			activities.add(result.value);

			toast.success("Activity created", {
				description: "The activity has been created successfully."
			});
		});

		open = false;
		clear();
	}
</script>

<Drawer.Root bind:open onClose={clear}>
	<Drawer.Trigger class={buttonVariants({ variant: "outline" })}>
		<Plus class="stroke-green-400" strokeWidth={3} /> New activity
	</Drawer.Trigger>
	<Drawer.Content>
		<Drawer.Header>
			<Drawer.Title>New activity</Drawer.Title>
			<Drawer.Description>Create a new activity to your day.</Drawer.Description>
		</Drawer.Header>

		<Separator />

		<div class="mb-4 flex flex-col gap-4">
			<div class="flex flex-col gap-1">
				<label for="name" class="text-base font-medium">Activity name</label>
				<Input id="name" placeholder="Learn Rust" maxlength={30} minlength={1} bind:value={name} />
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

		<Drawer.Footer class="flex-row justify-end gap-2">
			<Drawer.Close class={buttonVariants({ variant: "ghost" })}>Cancel</Drawer.Close>
			<Button class={cn(buttonVariants(), "bg-green-400 ring-foreground hover:bg-green-300")} onclick={handleSubmit}>Create</Button>
		</Drawer.Footer>
	</Drawer.Content>
</Drawer.Root>
