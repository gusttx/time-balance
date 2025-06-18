<script lang="ts">
	import { activities } from "$lib/activities.svelte";
	import { CircleDotDashed } from "@lucide/svelte";
	import * as Select from "./ui/select";

    let {
        value = $bindable()
    }: { 
        value?: string; 
    } = $props();

    let activity = $derived(activities.get(Number(value)));
</script>

<Select.Root
    type="single"
    bind:value
    disabled={activities.error || activities.length === 0}
>
    <Select.Trigger class="w-full max-w-50" size="sm">
        <div class="flex items-center gap-2 overflow-hidden">
            {#if activity}
                <CircleDotDashed />
                <p class="truncate">{activity.name}</p>
            {:else}
                Select activity
            {/if}
        </div>
    </Select.Trigger>
    <Select.Content>
        {#each activities.entries as activity (activity.id)}
            <Select.Item value={activity.id.toString()} class="">
                <CircleDotDashed />
                {activity.name}
            </Select.Item>
        {/each}
    </Select.Content>
</Select.Root>