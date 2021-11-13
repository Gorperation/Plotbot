// connect to bluetooth device

let connection: BluetoothRemoteGATTServer
let device: BluetoothDevice
let service: BluetoothRemoteGATTService
let characteristc: BluetoothRemoteGATTCharacteristic

let connectHandlers: Function[] = []
let disconnectHandlers: Function[] = []

export async function connect() {
	device = await navigator.bluetooth.requestDevice({
		filters: [{ services: ['cnc_service'] }],
	})
	console.log('> Found ' + device.name)

	connection = await device.gatt.connect()
	console.log('> Connected to ' + device.name)

	service = await connection.getPrimaryService('cnc_service')
	characteristc = await service.getCharacteristic('gcode')

	return true
}

// Connect handler
async function connected() {
	console.log('> Connected to ' + device.name)
	connectHandlers.forEach((fn) => fn())
}

// Add disconnect handler
async function onDisconnect(fn: Function) {
	disconnectHandlers.push(fn)
}

// Disconnect from bluetooth device
async function disconnect() {
	device?.gatt.disconnect()
}

// Disconnect handler
async function disconnected() {
	console.log('> Disconnected from ' + device.name)
	disconnectHandlers.forEach((fn) => fn())
}

// Send data to bluetooth device
export async function send(data: string) {}

// On data received from bluetooth device
export async function receive(data: string) {
	console.log('> Received: ' + data)
}
