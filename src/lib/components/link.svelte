<script lang="ts">
    import { openUrl } from '@tauri-apps/plugin-opener';
	import type { Snippet } from 'svelte';

    export function open(e: Event) {
        e.preventDefault();
        openUrl(href);
    }

    let {
        children,
        href
    }: {
        href: string;
        children: Snippet;
    } = $props();
</script>

<a class="underline" href={href} onclick={open}>
    {@render children()}
</a>