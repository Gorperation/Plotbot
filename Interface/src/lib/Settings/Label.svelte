<script lang="ts">
	export let text = 'Label'
	export let unit = ''
	export let top = false
</script>

<div class="container" class:top>
	<h3>{text}</h3>
	<span><slot /></span><span class="unit">{unit}</span>
</div>

<style lang="scss">
	.container {
		/* margin: 0.6em 0; */
		height: 2.2em;
		display: flex;
		flex-direction: row;
		align-items: top;
	}

	.container.top {
		z-index: 10;
		position: relative;
	}

	h3 {
		width: 6.8vw;
		display: inline-block;
		font-size: 1.05em;
		font-weight: 500;
		margin: 0;
		margin-top: 0.15em;
		margin-right: 0.5em;
	}

	span {
		display: inline-block;
	}

	.unit {
		font-size: 1.1em;
		font-weight: 500;
		margin-left: 0.2em;
	}
</style>
