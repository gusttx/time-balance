import { toast } from "svelte-sonner";
import { getAllActivities, type Activity } from "./commands/activity";
import { StateStatus } from "./utils";

class ActivitiesState {
	#activities = $state<Activity[]>([]);
	#status = $state(StateStatus.UNINITIALIZED);
	length = $derived(this.#activities.length);

	constructor() {
		this.load();
	}

	load() {
		if (this.#status === StateStatus.LOADING) {
			return;
		}

		this.#status = StateStatus.LOADING;

		getAllActivities().then((result) => {
			if (!result.ok) {
				const { error, message } = result.error;
				toast.error(`${error} error`, {
					description: message,
					action: {
						label: "Try again",
						onClick: () => this.load()
					}
				});

				this.#status = StateStatus.ERROR;
				return;
			}

			this.#activities = result.value;
			this.#status = StateStatus.LOADED;
		});
	}

	add(activity: Activity) {
		this.#activities.unshift(activity);
	}

	remove(id: number) {
		const index = this.#activities.findIndex((a) => a.id === id);
		if (~index) {
			this.#activities.splice(index, 1);
		}
	}

	edit(activity: Activity) {
		const index = this.#activities.findIndex((a) => a.id === activity.id);
		if (~index) {
			this.#activities[index] = activity;
		}
	}

	get(id: number): Activity | undefined {
		return this.#activities.find((a) => a.id === id);
	}

	get entries(): Activity[] {
		return this.#activities;
	}

	get loaded(): boolean {
		return this.#status === StateStatus.LOADED;
	}

	get loading(): boolean {
		return this.#status === StateStatus.LOADING || this.#status === StateStatus.UNINITIALIZED;
	}

	get error(): boolean {
		return this.#status === StateStatus.ERROR;
	}
}

export const activities = new ActivitiesState();
