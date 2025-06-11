<script lang="ts">
	import { Page, pages } from "$lib/navigation.svelte";
    import PageTemplate from "./page-template.svelte";

    const page = pages.get(Page.ENTRIES);
</script>

<PageTemplate header={{
    icon: page.icon,
    title: page.name,
    description: "Here you can list and manage your entries.",
    class: "bg-linear-[145deg] from-entry/30 to-entry/5"
}}>
    Todo
</PageTemplate>