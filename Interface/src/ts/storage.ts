import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import type { Status } from 'src/ts/utility'

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
	gcode: string
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
	error: null as string,
	drawing: null as Drawing,
	progress: 0,
})
