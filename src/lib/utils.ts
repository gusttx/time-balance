import { type } from '@tauri-apps/plugin-os';
import { clsx, type ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChild<T> = T extends { child?: any } ? Omit<T, 'child'> : T;
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type WithoutChildren<T> = T extends { children?: any } ? Omit<T, 'children'> : T;
export type WithoutChildrenOrChild<T> = WithoutChildren<WithoutChild<T>>;
export type WithElementRef<T, U extends HTMLElement = HTMLElement> = T & {
	ref?: U | null;
};

export const isDesktop = ['linux', 'windows', 'macos'].includes(type());

export type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
export function ok<T, E>(value: T): Result<T, E> {
	return { ok: true, value };
}
export function err<T, E>(error: E): Result<T, E> {
	return { ok: false, error };
}

export enum StateStatus {
	UNINITIALIZED,
	LOADING,
	LOADED,
	ERROR,
}