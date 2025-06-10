import { err, ok, type Result } from "$lib/utils";
import { invoke, type InvokeArgs, type InvokeOptions } from "@tauri-apps/api/core";

export type CommandError = {
    error: string;
    message: string;
};

export function isCommandError(error: unknown): error is CommandError {
    return (
        typeof error === "object" &&
        error !== null &&
        "error" in error &&
        "message" in error
    );
}

export type CommandPromise<T> = Promise<Result<T, CommandError>>;

export async function tryInvoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): CommandPromise<T> {
	try {
		const activity = await invoke<T>(cmd, args, options);
        return ok(activity);
	} catch (error) {
        console.error(error);

        if (isCommandError(error)) {
            return err(error);
        }

		return err({
			error: "Unknown",
			message: error instanceof Error ? error.message : "Unknown error"
		});
	}
}