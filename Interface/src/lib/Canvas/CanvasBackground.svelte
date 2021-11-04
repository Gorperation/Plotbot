<script lang="ts">
	// let canvas: HTMLDivElement;
	import { sineIn } from 'svelte/easing'
	const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms))

	let marching = false
	let style = 'dashed'
	let borderOffset = 0
	async function marchIn() {
		marching = true
		while (marching == true) {
			await delay(10)
			borderOffset += 0.1
		}
		for (let index = 28; index > 0; index--) {
			const p = sineIn(index / 28)
			await delay(10)
			borderOffset += p / 10
		}
	}
</script>

<div
	id="background"
	on:mouseenter={marchIn}
	on:mouseleave={() => (marching = false)}
>
	<svg width="100%" height="100%" xmlns="http://www.w3.org/2000/svg">
		{#if style == 'dashed'}
			<rect
				width="100%"
				height="100%"
				fill="none"
				stroke="white"
				stroke-width="2"
				stroke-dasharray="2, 4.5"
				stroke-dashoffset={borderOffset}
				stroke-linecap="square"
			/>
		{:else if style == 'blue'}
			<rect
				width="100%"
				height="100%"
				fill="none"
				stroke="#3300ff"
				stroke-width="8.6"
			/>
		{:else if style == 'red'}
			<rect
				width="100%"
				height="100%"
				fill="none"
				stroke="#ff4333"
				stroke-width="6.6"
				stroke-dasharray="4, 11"
				stroke-dashoffset={borderOffset}
				stroke-linecap="square"
			/>
		{/if}
	</svg>
</div>

<style lang="scss">
	#background {
		height: 100%;

		svg {
			transition: all 0.3s;
		}
	}
</style>
