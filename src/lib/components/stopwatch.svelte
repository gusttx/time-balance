<script lang="ts" module>
	import { CircleFadingPlus, Pause, Play, RotateCcw, Save } from "@lucide/svelte";
	import IconButton from "./icon-button.svelte";
	import { createEntry } from "$lib/commands/entry";
	import { entries } from "$lib/entries.svelte";
	import { toast } from "svelte-sonner";
	import ActivitySelect from "./activity-select.svelte";
	import { activities } from "$lib/activities.svelte";
	import * as Popover from "$lib/components/ui/popover/index.js";
	import Input from "./ui/input/input.svelte";
	import Button from "./ui/button/button.svelte";

	const MS_PER_SECOND = 1000;
	const MS_PER_MINUTE = 60 * MS_PER_SECOND;
	const MS_PER_HOUR = 60 * MS_PER_MINUTE;

	const maxTime = 99 * MS_PER_HOUR + 59 * MS_PER_MINUTE + 59 * MS_PER_SECOND + 999;

	function formatTime(num: number, pad = 2) {
		return Math.floor(num).toString().padStart(pad, "0");
	}

	function formatUnit(num: number, word: string) {
		return num === 1 ? `${num} ${word}` : `${num} ${word}s`;
	}

	function formatDuration(secs: number) {
		const h = Math.floor(secs / 3600);
		const m = Math.floor((secs % 3600) / 60);
		const s = secs % 60;

		const parts: string[] = [];
		if (h > 0) parts.push(formatUnit(h, "hour"));
		if (m > 0) parts.push(formatUnit(m, "minute"));
		if (s > 0) parts.push(formatUnit(s, "second"));

		const len = parts.length;

		return len === 0
			? "0 seconds"
			: len === 1
				? parts[0]
				: `${parts.slice(0, len - 1).join(", ")} and ${parts[len - 1]}`;
	}

	let time = $state(0);

	let selected: string | undefined = $state();
	let activity = $derived(activities.get(Number(selected)));
	let isSaving = $state(false);
	let animationFrame: number | null = $state(null);

	let hours = $derived(formatTime(time / MS_PER_HOUR));
	let minutes = $derived(formatTime((time / MS_PER_MINUTE) % 60));
	let seconds = $derived(formatTime((time / MS_PER_SECOND) % 60));
	let milliseconds = $derived(formatTime(time % MS_PER_SECOND, 3));

	let isRunning = $derived(animationFrame !== null);
	let canSave = $derived(activity && time >= 1 * MS_PER_SECOND && !isSaving);

	function start() {
		if (isRunning) return;

		let last = Date.now();
		animationFrame = requestAnimationFrame(function tick() {
			const now = Date.now();
			time = Math.min(time + now - last, maxTime);
			last = now;

			if (time >= maxTime) {
				pause();
				toast.info("Congratulations!", {
					description: "You reached the limit! 🥳🎉"
				});
				return;
			}

			animationFrame = requestAnimationFrame(tick);
		});
	}

	function pause() {
		if (isRunning) {
			cancelAnimationFrame(animationFrame!);
			animationFrame = null;
		}
	}

	function reset() {
		pause();
		time = 0;
	}

	async function save() {
		if (!canSave) return;

		isSaving = true;

		const result = await createEntry(Math.floor(time / MS_PER_SECOND), activity!.id);

		if (!result.ok) {
			isSaving = false;
			toast.error(`${result.error.error} error`, {
				description: result.error.message
			});
			return;
		}

		const title = "Created entry for " + activity!.name;
		const description = `Entry created with ${formatDuration(result.value.duration)}`;

		reset();
		isSaving = false;
		entries.add(result.value);
		toast.success(title, { description });
	}
</script>

<script lang="ts">
	function getTime(val: string, max: number) {
		const num = Number(val.replace(/([^0-9])/g, "").slice(-2));
		return Math.min(num, max).toString().padStart(2, "0");
	}

	let manualHours = $state("00");
	let manualMinutes = $state("00");
	let manualSeconds = $state("00");

	let disabled = $derived(!Number(manualHours) && !Number(manualMinutes) && !Number(manualSeconds));
	let open = $state(false); 

	function onHoursChange() {
		manualHours = getTime(manualHours, 99);
	}

	function onMinutesChange() {
		manualMinutes = getTime(manualMinutes, 59);
	}

	function onSecondsChange() {
		manualSeconds = getTime(manualSeconds, 59);
	}

	async function manualSave() {
		if (disabled || !activity) return;

		open = false;

		let secs = Number(manualHours) * 3600
			+ Number(manualMinutes) * 60
			+ Number(manualSeconds);

		manualHours = "00";
		manualMinutes = "00";
		manualSeconds = "00";

		const result = await createEntry(secs, activity!.id);

		if (!result.ok) {
			toast.error(`${result.error.error} error`, {
				description: result.error.message
			});
			return;
		}

		const title = "Created entry for " + activity!.name;
		const description = `Entry created with ${formatDuration(result.value.duration)}`;

		entries.add(result.value);
		toast.success(title, { description });
	}
</script>

<div class="flex flex-col gap-4">
	<div
		class="text-foreground font-rubik flex items-end justify-center text-[clamp(1.75rem,0.7016rem+8.3871vw,3.375rem)] leading-none font-semibold italic tabular-nums"
	>
		{#if time >= maxTime}
			<span class="not-italic opacity-20">🤯</span>
		{/if}
		<span>{hours}</span>
		<span class="text-muted-foreground">:</span>
		<span>{minutes}</span>
		<span class="text-muted-foreground">:</span>
		<span>{seconds}</span>
		<span
			class="text-muted-foreground relative bottom-[clamp(0.0625rem,-0.0585rem+0.9677vw,0.25rem)] text-[clamp(1rem,0.6774rem+2.5806vw,1.5rem)]"
		>
			.{milliseconds}
		</span>
	</div>

	<div class="flex w-full flex-wrap justify-center gap-2">
		<div class="flex gap-2">
			{#if animationFrame === null}
				<IconButton onclick={start} variant="outline" title="Start" disabled={time >= maxTime}>
					<Play class="text-yellow-400 drop-shadow-[0px_0px_3px_var(--color-yellow-400)]" />
				</IconButton>
			{:else}
				<IconButton onclick={pause} variant="outline" title="Pause">
					<Pause class="text-orange-400 drop-shadow-[0px_0px_3px_var(--color-orange-400)]" />
				</IconButton>
			{/if}
			<IconButton onclick={reset} variant="outline" title="Reset" disabled={time === 0}>
				<RotateCcw class="text-destructive drop-shadow-[0px_0px_3px_var(--color-destructive)]" />
			</IconButton>
			<IconButton onclick={save} variant="outline" title="Save" disabled={!canSave}>
				<Save class="text-entry drop-shadow-[0px_0px_3px_var(--color-entry)]" />
			</IconButton>
			<Popover.Root bind:open>
				<Popover.Trigger class="rounded-md" disabled={!selected}>
					<IconButton variant="outline" title="Add manual entry" disabled={!selected}>
						<CircleFadingPlus
							class="text-green-400 drop-shadow-[0px_0px_3px_var(--color-green-400)]"
						/>
					</IconButton>
				</Popover.Trigger>
				<Popover.Content class="bg-background/85 bg-linear-[135deg] from-input/0 via-input/20 to-input/0 flex flex-col gap-2 backdrop-blur-xs max-w-50">
					<div class="flex items-center justify-center gap-0.5 text-lg leading-0">
						<Input
							oninput={onHoursChange}
							bind:value={manualHours}
							aria-label="Hours"
							class="h-8 max-w-8 p-0 text-center"
						/>
						<span class="text-muted-foreground">:</span>
						<Input
							oninput={onMinutesChange}
							bind:value={manualMinutes}
							aria-label="Minutes"
							class="h-8 max-w-8 p-0 text-center"
						/>
						<span class="text-muted-foreground">:</span>
						<Input
							oninput={onSecondsChange}
							bind:value={manualSeconds}
							aria-label="Seconds"
							class="h-8 max-w-8 p-0 text-center"
						/>
					</div>

					<Button
						class="grow text-green-400! text-shadow-[0px_0px_6px_var(--color-green-400)]"
						variant="outline"
						size="sm"
						{disabled}
						onclick={manualSave}
					>
						<Save class="text-green-400 drop-shadow-[0px_0px_3px_var(--color-green-400)]" />
						Save
					</Button>
				</Popover.Content>
			</Popover.Root>
		</div>
		<ActivitySelect bind:value={selected} />
	</div>
</div>
