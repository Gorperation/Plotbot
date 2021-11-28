<script lang="ts">
	import { status as statusStore } from 'src/ts/storage'
	import { renderTime } from 'src/ts/utility'
	import Button from '../Button.svelte'
	import { Status } from 'src/ts/utility'
	import { slide } from 'src/ts/animations'

	export let drawing

	function click() {
		drawing.status = Status.Drawing
		statusStore.set({
			printing: true,
			error: null,
			drawing,
			progress: 0,
		})
	}
</script>

<div class="container" transition:slide={{ duration: 350 }}>
	<div class="stats">
		<div class="col">
			<span>Status</span>
			<span class="status {Status[drawing.status].toLowerCase()}"
				>{Status[drawing.status]}</span
			>
		</div>
		<div class="col">
			<span>Size</span>
			<span class="size">{drawing.sizeKB} kB</span>
		</div>
		<div class="col">
			<span>Time</span>
			<span class="time">{renderTime(drawing.time)}</span>
		</div>
	</div>
	<div class="image">
		{#if drawing.status != Status.Drawing}
			<span out:slide={{ axis: 'X' }}>
				<Button class="button" size="1.1" {click}
					>{drawing.status == Status.Ready
						? 'DRAW'
						: 'REDRAW'}</Button
				>
			</span>
		{/if}
	</div>
</div>

<style lang="scss">
	.container {
		border: 1px solid #ccc;
		padding: 0.5em;
		padding-left: 0.6em;
		/* padding-bottom: 1em; */
		position: relative;
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

		.finished,
		.ready {
			color: hsl(120, 60%, 59%);
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
