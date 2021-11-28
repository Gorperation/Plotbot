import { drawings } from 'src/ts/storage'
import { Status } from 'src/ts/utility'
import { status } from 'src/ts/storage'

let gcode: string[]
let line = 0
let timer: NodeJS.Timeout
let printing = false

function iter() {
	if (gcode[line].startsWith(';')) return
	console.log(gcode[line])
	status.update((s) => {
		s.progress = Math.floor((line / gcode.length) * 100)
		return s
	})

	line++
	if (line >= gcode.length) {
		clearInterval(timer)
		printing = false

		status.update((s) => {
			s.printing = false
			return s
		})
		drawings.update((d) => {
			const drawing = d.find((d) => d.status === Status.Drawing)
			if (drawing) drawing.status = Status.Finished
			return d
		})
	}
}

status.subscribe((s) => {
	if (printing) return
	if (s.printing) {
		printing = true
		if (s.drawing) {
			timer = setInterval(iter, 50)
		} else {
			printing = false
			status.update((s) => {
				s.printing = false
				return s
			})
		}
	} else if (timer != undefined) {
		clearInterval(timer)
		timer = undefined
	}
})

export function queueDrawing(gcodeString: string) {
	gcode = gcodeString.split('\n')
	const sizeKB = Math.ceil(gcodeString.length / 1024)
	drawings.update((d) => [
		...d,
		{ status: Status.Ready, sizeKB, time: 0, gcode: gcodeString },
	])
}
