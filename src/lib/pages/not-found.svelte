<script lang="ts">
	import { FileQuestion } from "@lucide/svelte";
    import Page from "./page.svelte";
</script>

<Page
    header={{
        icon: FileQuestion,
        title: "Not Found",
        description: "The requested page was not found.",
        class: "bg-linear-[145deg] from-destructive/15 to-transparent"
    }}
>
    <!-- TODO Maybe an easteregg? -->
</Page>