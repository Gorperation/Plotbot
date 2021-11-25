<script lang="ts">
	import { connect } from './bluetooth'

	let size = 0.9

	function click(e: MouseEvent) {
		connect()
	}
</script>

<svg
	on:click={click}
	width={197 * size}
	height={172 * size}
	viewBox="0 0 197 172"
	fill="none"
	xmlns="http://www.w3.org/2000/svg"
>
	<!-- <path
		d="M64.1101 43.795L109.924 84.7015L51.5911 103.925L64.1101 43.795Z"
		fill="#FFC700AA"
	/> -->
	<path
		d="M54.2571 105.179L66.4231 46.7448L110.945 86.4977L54.2571 105.179Z"
		stroke="white"
	/>
	<rect x="134.931" y="41.7366" width="35" height="35" stroke="white" />
	<circle cx="135.931" cy="117.737" r="16" stroke="white" />
	<circle cx="19.9314" cy="61.7366" r="12" stroke="white" />
</svg>

<style lang="scss">
	svg {
		/* display: none; */
		position: absolute;
		right: 0em;
		top: 1.1em;
		transition: all 0.4s;
	}

	@media screen and (max-width: 68em) {
		svg {
			transform: translateX(calc(60em - 80vw));
		}
	}
</style>
