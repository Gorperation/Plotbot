<script lang="ts">
	export let size: string | number = '1'
</script>

<button style="transform: scale({size});"><slot /></button>

<style lang="scss">
	button {
		height: 2.5em;
		width: 6em;

		cursor: pointer;
		background: black;
		color: white;
		border: 1px solid white;
		box-sizing: border-box;
		transition: all 0.2s cubic-bezier(0.33, 1, 0.68, 1);

		&:hover {
			background: rgb(240, 240, 240);
			color: black;
		}

		&:active {
			background: rgb(226, 226, 226);
		}
	}
</style>
