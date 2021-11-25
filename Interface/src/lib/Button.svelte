<script lang="ts">
	export let size: string | number = '1'
	export let click = () => {}
</script>

<button
	style="transform: scale({size});"
	class={$$props.class}
	on:click={click}
>
	<slot />
</button>

<style lang="scss">
	button {
		height: 2.05rem;
		width: 5.1rem;

		font-family: 'Roboto', sans-serif;
		font-weight: 500;
		font-size: 0.77em;
		letter-spacing: 0.07em;

		display: inline-block;

		cursor: pointer;
		background: black;
		color: white;
		/* border: none; */
		border: 1px solid hsl(0, 0%, 92%);
		box-sizing: border-box;
		transition: all 0.3s cubic-bezier(0.33, 1, 0.68, 1);

		&:hover {
			background: hsl(0, 0%, 99%);
			border: 2px solid white;
			color: black;
		}
	}
</style>
