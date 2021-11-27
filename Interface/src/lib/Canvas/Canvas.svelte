<script lang="ts">
	import Button from './../Button.svelte'
	import FigmaView from './FigmaView.svelte'
	import CanvasBackground from './CanvasBackground.svelte'
	import { copyFigmaArtboard, loadFigmaURL } from 'src/clipboard'
	import { drawings, figmaDoc, get, options } from 'src/storage'
	import { popToast } from '../Toasts/toast'
	import { sendGcode } from '../gcode'

	let queue: () => void, marching: boolean
	let svgData: string

	const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms))

	async function print() {
		popToast('Preparing to plot your design...')

		let data: any = get(options)
		console.log('Getting store in print()', data)
		data.svg = svgData

		const res = await fetch('https://plotbot.art/api/cam', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json',
			},
			body: JSON.stringify(data),
		})
		const gcode = await res.text()

		console.log(gcode)
		// window.location.href =
		// 	'data:application/octet-stream,' + encodeURIComponent(gcode)

		const lines = gcode.split('\n')
		console.log(lines.length, 'lines')
		sendGcode(lines)

		// drawings.update((list) => [...list, svgData])
	}
</script>

<div class="container">
	<div class="buttons">
		<Button click={print}>Print</Button>
		<Button click={loadFigmaURL}>Refresh</Button>
		<Button>Mark</Button>
		<Button click={copyFigmaArtboard}>Copy</Button>
	</div>
	<div
		id="canvas"
		on:mouseenter={() => (marching = true)}
		on:mouseleave={() => (marching = false)}
	>
		<CanvasBackground bind:marching />
		<!-- <canvas bind:this={canvas} /> -->
		<FigmaView bind:svgData />
	</div>
</div>

<style lang="scss">
	.container {
		height: calc(100% - 2em);
		position: relative;
		/* top: -5em; */
		display: inline-flex;
		flex-direction: column;
		justify-content: center;

		.buttons {
			margin-bottom: 0.3em;
			display: flex;
			gap: 0.3em;
		}

		#canvas {
			background-color: black;
			width: min(225vh * 0.3, 225vw * 0.18);
			height: min(265vh * 0.3, 265vw * 0.18);
			position: relative;
		}
	}
</style>
