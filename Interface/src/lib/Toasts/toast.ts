import { type Writable, writable } from 'svelte/store'

const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms))

export type Toast = {
	message: string
	type: 'success' | 'error' | 'info'
	timeout: number
}

export const toastStack: Writable<Toast[]> = writable([])

export async function popToast(
	message: string,
	type: 'success' | 'info' | 'error' = 'info',
	timeout = 0
) {
	timeout = message.length * 90
	const toast: Toast = { message, type, timeout }
	toastStack.update((toasts) => [...toasts, toast])
	await delay(timeout)
	toastStack.update((toasts) => toasts.filter((i) => i !== toast))
}
