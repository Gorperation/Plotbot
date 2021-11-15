<script lang="ts">
	import { renderTime } from 'src/utility'
	import Button from '../Button.svelte'
	import { Status } from './index.d'

	export let name: string
	export let sizeKB: number
	export let status: Status
	export let time: number
</script>

<div id="container">
	<div class="stats">
		<div class="col">
			<span>Status</span>
			<span class="status {Status[status].toLowerCase()}"
				>{Status[status]}</span
			>
		</div>
		<div class="col">
			<span>Size</span>
			<span class="size">{sizeKB} kB</span>
		</div>
		<div class="col">
			<span>Time</span>
			<span class="time">{renderTime(time)}</span>
		</div>
	</div>
	<div class="image">
		{#if status != Status.Drawing}
			<Button class="button" size="1.1">REDRAW</Button>
		{/if}
	</div>
</div>

<style lang="scss">
	#container {
		border: 1px solid #ccc;
		padding: 0.5em;
		padding-left: 0.7em;
		/* padding-bottom: 1em; */
		display: flex;
		gap: 1em;
	}

	.image {
		/* background-image: linear-gradient(45deg, #000000 26.92%, #ffffff 26.92%, #ffffff 50%, #000000 50%, #000000 76.92%, #ffffff 76.92%, #ffffff 100%); */
		/* background-size: 18.38px 18.38px; */
		display: inline-block;
		width: 10em;
		flex: 1;
		display: flex;
		flex-direction: row;
		align-items: center;
		justify-content: right;
		padding-right: 0.4em;

		:global(.button) {
			margin-right: 0.5em;
			/* margin-right: -1.5em; */
		}
	}

	.stats {
		/* display: inline-block; */
		.col span:first-child {
			width: 4em;
		}
		span {
			color: #f3f3f3;
			display: inline-block;
		}

		.finished {
			color: rgb(112, 202, 112);
		}
		.drawing {
			color: #ffc700;
		}
		.canceled,
		.failed {
			color: tomato;
		}

		.size {
			font-family: 'Roboto Mono', monospace;
			font-size: 0.9em;
		}
	}
</style>
