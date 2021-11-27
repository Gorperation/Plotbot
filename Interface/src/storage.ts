import { type Writable, writable } from 'svelte/store'
import type { Status } from './lib/Panel'

export function get<T>(store: Writable<T>) {
	let val: T
	store.subscribe(($) => (val = $))()
	return val
}

// export const device: Writable<BluetoothDevice> = writable(undefined)
export const figmaDoc = writable({ file: '', name: '', node: '' })

type Drawing = {
	status: Status
	sizeKB: number
	time: number
}

export const drawings: Writable<Drawing[]> = writable([])

export const options = writable({
	fill_density: 0,
	fill_pattern: 'monotonic',
	fill_connected: true,
	fill_overlap: 100,
	fill_angle: 45,
	fill_speed: 60,
	perimeters: 1,
	perimeter_speed: 60,
})

export const status = writable({
	printing: false,
	paused: false,
	error: false as boolean | string,
	progress: 0,
	fileName: '',
	fileSize: 0,
})
