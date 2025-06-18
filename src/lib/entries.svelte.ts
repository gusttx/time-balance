import { toast } from "svelte-sonner";
import { getEntriesPage, type Entry } from "./commands/entry";
import { StateStatus } from "./utils";

class EntryState {
	#entries = $state<Entry[]>([]);
	#total = $state(0);
	#page = $state(1);
	#status = $state(StateStatus.UNINITIALIZED);

	constructor() {
		this.loadPage(this.#page);
	}

	async loadPage(page: number) {
		if (this.#status === StateStatus.LOADING) {
			return;
		}

		this.#status = StateStatus.LOADING;

		const result = await getEntriesPage(page);

		if (!result.ok) {
			const { error, message } = result.error;
			toast.error(`${error} error`, {
				description: message,
				action: {
					label: "Try again",
					onClick: () => this.loadPage(page)
				}
			});

			this.#status = StateStatus.ERROR;
			return;
		}

		this.#entries = result.value[0];
		this.#total = result.value[1];
		this.#page = page;
		this.#status = StateStatus.LOADED;
	}

	add(entry: Entry) {
		this.#total += 1;
		if (this.#page === 1) {
			this.#entries = [entry, ...this.#entries.slice(0, 9)];
		} else {
			this.loadPage(this.#page);
		}
	}

	get page(): number {
		return this.#page;
	}

	set page(page: number) {
		this.loadPage(page);
	}

	get total(): number {
		return this.#total;
	}

	get entries(): Entry[] {
		return this.#entries;
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

export const entries = new EntryState();
