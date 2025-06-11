<script lang="ts">
	import CreateActivity from "$lib/components/create-activity.svelte";
	import PageTemplate from "./page-template.svelte";
	import EditActivities from "$lib/components/edit-activities.svelte";
	import { Page, pages } from "$lib/navigation.svelte";

	const page = pages.get(Page.ACTIVITIES);
</script>

<PageTemplate
	header={{
		icon: page.icon,
		title: page.name,
		description: "Here you can create and manage your activities.",
		class: "bg-linear-[155deg] from-activity/30 to-activity/2.5"
	}}
>
	<div class="flex justify-end">
		<CreateActivity />
	</div>

	<EditActivities />
</PageTemplate>
