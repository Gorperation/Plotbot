<script lang="ts">
	import { onMount } from 'svelte'
	import CanvasBackground from './CanvasBackground.svelte'
	// import { WebGLPreview } from 'gcode-preview'
	// import * as THREE from 'three'

	let canvas: HTMLCanvasElement
	// WebGLPreview.

	onMount(() => {
		// const preview = new WebGLPreview({
		// canvas,
		// })
		// if (response.status !== 200) {
		// 	console.error('ERROR. Status Code: ' + response.status)
		// 	return
		// }
		// const gcode = await response.text()
		// preview.processGCode(gcode)
	})
</script>

<div id="container">
	<div id="canvas">
		<CanvasBackground />
		<!-- <canvas bind:this={canvas} /> -->
	</div>
</div>

<style lang="scss">
	#container {
		width: 34%;
		height: calc(100% - 2em);
		margin-left: 3.5em;
		position: absolute;
		display: inline-flex;
		flex-direction: column;
		justify-content: center;

		#canvas {
			background-color: black;
			width: 60vh * 1;
			height: 80vh * 1;
		}
	}
</style>
