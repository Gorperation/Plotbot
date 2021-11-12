export function renderTime(seconds: number): string {
	let minutes = Math.floor(seconds / 60)
	if (minutes) return `${minutes}m ${seconds}s`
	else return `${seconds}s`
}
