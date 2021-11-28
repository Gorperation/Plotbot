<script lang="ts">
	import Toasts from './lib/Toasts/Toasts.svelte'
	import Settings from './lib/Settings/Settings.svelte'
	import Doodads from './lib/Doodads.svelte'
	import Canvas from './lib/Canvas/Canvas.svelte'
	import Background from './lib/Background.svelte'
	import Tag from './lib/Tag.svelte'
	import Panel from './lib/Panel/Panel.svelte'
	import Scroller from './lib/Scroller.svelte'
	import Terminal from './lib/Terminal.svelte'
	import { init } from './ts/websockets'

	init()

	// Must append mm to width and height attr of SVG
</script>

<main>
	<Background />
	<Panel />
	<Canvas />
	<Settings />
	<Tag />
	<Toasts />
	<Terminal />
</main>
<Scroller />
<Doodads />

<style lang="scss">
	:root {
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto,
			Oxygen, Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
	}

	main {
		margin-top: 2em;
		height: calc(100vh - 2em);
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: 3em;
	}
</style>
