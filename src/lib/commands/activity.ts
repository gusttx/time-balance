import { tryInvoke, type CommandPromise } from ".";

export type Activity = {
	id: number;
	name: string;
	points_per_second: number;
};

export async function getAllActivities(): CommandPromise<Activity[]> {
	return tryInvoke("get_all_activities");
}

export async function createActivity(
	name: string,
	pointsPerSecond: number
): CommandPromise<Activity> {
	return tryInvoke("create_activity", { name, pointsPerSecond });
}

export async function deleteActivity(id: number): CommandPromise<void> {
	return tryInvoke("delete_activity", { id });
}

export async function editActivity(
	id: number,
	name: string,
	pointsPerSecond: number
): CommandPromise<Activity> {
	return tryInvoke("edit_activity", { id, name, pointsPerSecond });
}