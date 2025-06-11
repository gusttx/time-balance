<script lang="ts">
	import { AlarmClock, Plus, Scale } from "@lucide/svelte";
	import PageTemplate from "./page-template.svelte";
	import { cn } from "$lib/utils";
	import Button from "$lib/components/ui/button/button.svelte";
	import { navigation, Page, pages, type PageInfo } from "$lib/navigation.svelte";

    const home = pages.get(Page.HOME);
    const activity = pages.get(Page.ACTIVITIES);
    const entry = pages.get(Page.ENTRIES);
</script>

{#snippet button(page: PageInfo, className?: string)}
    <Button variant="outline" size="lg" class="w-full max-w-50 border-2" onclick={() => navigation.navigate(page.page)}>
        <page.icon class={className}/> {page.name}
    </Button>
{/snippet}

<PageTemplate
    header={{
        icon: home.icon,
        title: home.name,
        description: "Welcome to the home page!",
        class: "bg-linear-[155deg] from-activity2/40 via-activity/35 to-entry/30"
    }}
>
    <div class="h-full flex flex-col justify-between">
        <div></div>
        <div class="flex flex-col gap-4 relative">
            {#each [1, 2] as i}
                <h2 class={cn(
                    "text-[clamp(2.5rem,0.1596rem+13.617vw,4.5rem)] leading-none font-bold font-rubik uppercase flex flex-col w-full items-center bg-linear-[135deg] from-activity to-activity2 text-transparent bg-clip-text",
                    { "blur-[8px] absolute scale-103": i === 1 }
                )}>
                    <span>Time</span>
                    <span>Balance</span>
                </h2>
            {/each}
    
            <div class="flex justify-center items-center gap-2">
                <AlarmClock size={32} strokeWidth={2.8} class="stroke-entry"/>
                <Plus size={24} strokeWidth={3} class="stroke-muted-foreground" />
                <Scale size={32} strokeWidth={2.4} class="stroke-entry2" />
            </div>
    
            <p class="text-muted-foreground text-center italic">
                Time Balance is a time tracking app that allows you to track your time and balance it with your activities.
            </p>
    
            <div class="flex justify-center gap-4 w-full flex-wrap">
                {@render button(activity, "stroke-activity2")}
                {@render button(entry, "stroke-entry")}
            </div>
        </div>
        <footer class="text-sm text-muted-foreground italic font-dosis text-center mt-4">
            Made with ❤️ by <a href="https://github.com/gusttx" class="underline">gusttx</a>
        </footer>
    </div>
</PageTemplate>