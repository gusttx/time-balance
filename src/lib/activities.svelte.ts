import { toast } from "svelte-sonner";
import { getAllActivities, type Activity } from "./commands/activity";

export enum ActivitiesStatus {
    UNINITIALIZED,
    LOADING,
    LOADED,
    ERROR,
}

class ActivitiesState {
    #activities = $state<Activity[]>([]);
    #status = $state(ActivitiesStatus.UNINITIALIZED);

    constructor() {
        this.load();
    }

    load() {
        if (this.#status === ActivitiesStatus.LOADING) {
            return;
        }

        this.#status = ActivitiesStatus.LOADING;
        
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

                this.#status = ActivitiesStatus.ERROR;
                return;
            }

            this.#activities = result.value;
            this.#status = ActivitiesStatus.LOADED;
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

    get length(): number {
        return this.#activities.length;
    }

    get entries(): Activity[] {
        return this.#activities;
    }

    get loaded(): boolean {
        return this.#status === ActivitiesStatus.LOADED;
    }

    get loading(): boolean {
        return this.#status === ActivitiesStatus.LOADING || this.#status === ActivitiesStatus.UNINITIALIZED;
    }

    get error(): boolean {
        return this.#status === ActivitiesStatus.ERROR;
    }
}

export const activities = new ActivitiesState();