import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import { success, error, info, persistent } from 'src/ts/toast'
import { delay } from './utility'

let socket: WebSocket
let listeners: Function[] = []
let lastPing: number
let clearNoResponse: Function

export function init() {
	setInterval(() => {
		if (socket && socket.readyState === socket.OPEN) {
			if (Date.now() - lastPing > 12000) {
				if (clearNoResponse) return
				clearNoResponse = persistent('No response from Plotbot')
			} else {
				if (clearNoResponse) {
					success('Plotbot is responding again')
					clearNoResponse()
					clearNoResponse = undefined
				}
			}
		}
	}, 5000)

	connect()
}

export async function onmessage(fn: Function) {
	listeners.push(fn)
}

export async function offmessage(fn: Function) {
	listeners = listeners.filter((l) => l !== fn)
}

export function sendMessage(message: string) {
	if (!socket) return false
	const encoder = new TextEncoder()
	const encoded = encoder.encode(message)
	socket.send(encoded)
}

export async function connect() {
	const ip = await getIP()
	await _connect(`ws://${ip}:81`)
	success('Connected to Plotbot')

	socket.onerror = () => error('Connection error!', 5000)
	socket.onclose = () => error('Connection dropped!', 5000)
	socket.onmessage = (event) => {
		lastPing = Date.now()
		listeners.forEach((fn) => fn(event))
	}
}

async function getIP() {
	try {
		const res = await fetch('http://plotbot.local/ip')
		return res.text()
	} catch (error) {
		await delay(1000)
		const res = await fetch('http://plotbot.local/ip')
		return res.text()
	}
	error('Could not find IP address', 5000)
}

const _connect = (url: string): Promise<void> =>
	new Promise((resolve) => {
		socket = new WebSocket(url)
		socket.binaryType = 'arraybuffer'
		socket.addEventListener('open', () => resolve())
	})
