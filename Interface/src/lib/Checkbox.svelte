<script lang="ts">
	export let value = false
</script>

<div>
	<label class="container">
		<input type="checkbox" bind:checked={value} />
		<span class="checkmark" />
	</label>
</div>

<style lang="scss">
	.container {
		display: inline-block;
		position: relative;
		padding-left: 1.8em;
		cursor: pointer;
		font-size: 1.2em;
		user-select: none;
	}

	/* Hide the browser's default checkbox */
	.container input {
		opacity: 0;
		cursor: pointer;
		height: 0;
		width: 0;
	}

	/* Create a custom checkbox */
	.checkmark {
		position: absolute;
		top: 0;
		left: 0;
		$size: 1em;
		width: $size;
		height: $size;
		border: 2px solid hsl(0, 0%, 92%);
		transition: all 0.15s cubic-bezier(0.25, 1, 0.5, 1);
	}

	/* On mouse-over, add a grey background color */
	.container:hover input ~ .checkmark {
		border: 2px solid hsl(0, 0%, 99%);
		/* border: 3px solid white; */
	}

	/* When the checkbox is checked, add a blue background */
	.container input:checked ~ .checkmark {
		border: 2px solid hsl(0, 0%, 99%);
		background-color: hsl(0, 0%, 99%);
	}

	/* Create the checkmark/indicator (hidden when not checked) */
	.checkmark:after {
		content: '';
		position: absolute;
		display: none;
	}

	/* Show the checkmark when checked */
	.container input:checked ~ .checkmark:after {
		transform: rotate(45deg) scale(1);
	}

	/* Style the checkmark/indicator */
	.container .checkmark:after {
		display: block;
		left: 0.3em;
		top: 0.05em;
		width: 5px;
		height: 10px;
		border: solid black;
		border-width: 0 3px 3px 0;
		transform: rotate(45deg) scale(0.6);
		transform-origin: center;
		transition: transform 0.3s cubic-bezier(0.25, 1, 0.5, 1);
	}
</style>
