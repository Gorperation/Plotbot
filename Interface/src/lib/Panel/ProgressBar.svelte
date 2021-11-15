<script lang="ts">
	export let progress = 1
</script>

<div class="container" style="--progress:{progress * 100}%">
	<div class="bar"><div class="progress" /></div>
	<span class="percent">{Math.floor(progress * 100)}%</span>
</div>

<style lang="scss">
	.container {
		position: relative;
	}
	span {
		font-size: 0.9em;
		height: 0.4em;
		display: inline-block;
	}

	.bar {
		/* border-bottom: 1px solid #ffc700; */
		/* border-radius: 2em; */
		width: calc(100% - 2.6em);
		position: relative;
		height: 0.6em;

		.progress {
			width: var(--progress);
			height: 0.45em;
			float: left;
			position: absolute;
			top: 0.2em;
			/* left: -0.13em; */
			/* border: 2.4px solid black; */
			/* border-radius: 15px; */
			background-color: #ffc700;
			transition: all 0.3s;
		}
	}

	.percent {
		position: absolute;
		right: 0;
		color: #eeeeee;
		top: -0.2em;
	}
</style>
