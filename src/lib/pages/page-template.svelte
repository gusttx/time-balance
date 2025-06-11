<script lang="ts">
	import Separator from "$lib/components/ui/separator/separator.svelte";
	import { cn } from "$lib/utils";
	import type { IconProps } from "@lucide/svelte";
	import type { Component, Snippet } from "svelte";

    let {
        children,
        header,
    }: {
        header?: {
            icon: Component<IconProps>,
            title: string,
            description: string,
            class?: string
        }
        children?: Snippet
    } = $props();
</script>

<div class="flex flex-col overflow-auto no-scrollbar grow">
    {#if header}
        <header class={cn(header.class, "p-4")}>
            <h1 class="flex items-center justify-center gap-2 text-xl font-rubik">
                <span class={`rounded-md`}>
                    <header.icon size={20} />
                </span>
                <span class="text-foreground leading-none">
                    {header.title}
                </span>
            </h1>
            <p class="text-muted-foreground text-center break-words hyphens-auto font-dosis italic">{header.description}</p>
        </header>
        <Separator />
    {/if}

    <div class="bg-grid relative grow flex flex-col">
        <div class="bg-grid-background size-full absolute"></div>
        <div class="relative z-1 p-4 grow">
            {@render children?.()}
        </div>
    </div>
</div>