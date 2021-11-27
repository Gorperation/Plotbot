let gcodeFile: string
let line = 0

export function sendGcode(gcode: string) {
	gcodeFile = gcode
	async function sendLines(lines: string[]) {
		for (const line of lines) {
			if (line.startsWith(';')) return
			console.log(line)
			// await delay(50)
			// await fetch(
			// 	`http://plotbot.local/command?commandText=${line}&PAGEID=2`
			// )
		}
	}
}
