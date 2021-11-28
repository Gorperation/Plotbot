import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { delay } from 'src/ts/utility'

export type Toast = {
	message: string
	type: 'success' | 'error' | 'info'
}

export const toastStack: Writable<Toast[]> = writable([])

async function popToast(
	message: string,
	type: 'success' | 'info' | 'error',
	timeout: number = null
) {
	if (timeout == null) timeout = 500 + message.length * 90
	const toast: Toast = { message, type }
	toastStack.update((toasts) => [...toasts, toast])
	await delay(timeout)
	toastStack.update((toasts) => toasts.filter((i) => i !== toast))
}

export function success(message: string, timeout = null) {
	popToast(message, 'success', timeout)
}

export function info(message: string, timeout = null) {
	popToast(message, 'info', timeout)
}

export function error(message: string, timeout = null) {
	popToast(message, 'error', timeout)
}

export function persistent(
	message: string,
	type: 'success' | 'info' | 'error' = 'info'
) {
	const toast: Toast = { message, type }
	toastStack.update((toasts) => [...toasts, toast])
	return () =>
		toastStack.update((toasts) => toasts.filter((i) => i !== toast))
}
