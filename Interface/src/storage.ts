import { writable } from 'svelte/store'

// export const device: Writable<BluetoothDevice> = writable(undefined)

export const status = writable({
	printing: false,
	paused: false,
	error: false as boolean | string,
	progress: 0,
	fileName: '',
	fileSize: 0,
})
