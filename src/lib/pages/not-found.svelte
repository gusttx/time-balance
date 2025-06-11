<script lang="ts">
	import { FileQuestion } from "@lucide/svelte";
    import PageTemplate from "./page-template.svelte";
</script>

<PageTemplate
    header={{
        icon: FileQuestion,
        title: "Not Found",
        description: "The requested page was not found.",
        class: "bg-linear-[145deg] from-destructive/15 to-transparent"
    }}
>
    <!-- TODO Maybe an easteregg? -->
</PageTemplate>