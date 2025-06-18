import { tryInvoke, type CommandPromise } from ".";

export type Entry = {
	id: number;
	duration: number;
	points_per_second: number;
	activity_id: number;
	activity_name: string;
};

export async function createEntry(duration: number, activityId: number): CommandPromise<Entry> {
	return tryInvoke("create_entry", { duration, activityId });
}

export async function getEntriesPage(page: number): CommandPromise<[Entry[], number]> {
	return tryInvoke("get_entry_page", { page: page });
}

export async function deleteEntry(id: number): CommandPromise<void> {
	return tryInvoke("delete_entry", { id });
}
