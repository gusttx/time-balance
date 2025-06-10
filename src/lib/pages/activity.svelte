<script lang="ts">
	import CreateActivity from "$lib/components/create-activity.svelte";
	import { Dumbbell } from "@lucide/svelte";
	import Page from "./page.svelte";
	import EditActivities from "$lib/components/edit-activities.svelte";
</script>

<Page
	header={{
		icon: Dumbbell,
		title: "Activities",
		description: "Here you can create and manage your activities.",
		class: "bg-linear-[145deg] from-activity/30 to-activity/5"
	}}
>
	<div class="flex justify-end">
		<CreateActivity />
	</div>

	<EditActivities />
</Page>
