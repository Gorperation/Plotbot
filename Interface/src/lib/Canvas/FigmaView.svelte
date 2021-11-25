<script lang="ts">
	import { figmaDoc } from 'src/storage'
	import { popToast } from 'src/lib/Toasts/toast'
	import { onMount } from 'svelte'
	import { LoadingManager } from 'three'

	// figmaDoc.subscribe(async (f) => {
	// popToast(f.toString())

	// })

	let svgContainer = document.createElement('div')
	let svg: SVGSVGElement
	export let svgData: string
	$: svgData = svg?.outerHTML

	const figmaDoc = {
		file: 'sHTKQ6PDqOb31azEa524nk',
		name: 'Plotbot-Test',
		node: '2:2',
	}

	async function load() {
		let res = await fetch(
			`https://api.figma.com/v1/images/${figmaDoc.file}?ids=${figmaDoc.node}&format=svg&svg_include_id=true`,
			{
				headers: {
					'X-Figma-Token':
						'278653-ba7bb26d-9065-40f9-8b21-84428f2e67a6',
				},
			}
		)
		const data = await res.json()
		const url = data.images[figmaDoc.node]
		res = await fetch(url)
		const svgData = await res.text()
		modifySVG(svgData)
	}

	function modifySVG(svgData: string) {
		svgContainer.innerHTML = svgData
		console.log(svgContainer)
		svg = svgContainer.querySelector('svg') as SVGSVGElement
		if (!svg) {
			popToast('No svg found in Figma Response', 'error')
			return
		}

		svg.setAttribute('width', '100%')
		svg.setAttribute('height', '100%')
		const viewbox = svg.getAttribute('viewBox').split(' ')
		const rect: SVGRectElement = svg.querySelector('rect')
		if (
			rect.getAttribute('fill') == 'white' &&
			rect.getAttribute('width') === viewbox[2] &&
			rect.getAttribute('height') === viewbox[3]
		)
			rect.remove()
	}

	onMount(() => load())
</script>

<div>
	{#if svg}
		{@html svg.outerHTML}
	{:else}
		Loading...
	{/if}
</div>

<style lang="scss">
	div {
		position: absolute;
		top: 0;
		width: calc(100% - 4px);
		height: calc(100% - 4px);
		padding: 2px;
	}
</style>
