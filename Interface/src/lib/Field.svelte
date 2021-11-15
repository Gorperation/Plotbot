<script lang="ts">
	import { onMount } from 'svelte'

	export let size = '10'
	export let placeholder = ''
	export let value = ''
	export let length: number = null
	export let max: number = null
	export let type = 'text'

	let input: HTMLInputElement

	function onInput() {
		if (input.value.length > length)
			input.value = input.value.slice(0, length)
	}

	function onBlur(event: FocusEvent) {
		if (max && parseFloat(input.value) > max) event.preventDefault()
	}

	onMount(() => (input.type = type))
</script>

<input
	{placeholder}
	bind:value
	style="--size:{size}em;"
	on:blur={onBlur}
	on:input={onInput}
	bind:this={input}
/>

<style lang="scss">
	input {
		font-family: 'Roboto', sans-serif;
		font-weight: 500;
		font-size: 0.87em;
		line-height: 1.3em;
		letter-spacing: 0.07em;
		padding: 0.1em 0.25em;

		width: var(--size);

		background: black;
		color: white;
		/* border: none; */
		border: 2px solid hsl(0, 0%, 92%);
		transition: all 0.3s cubic-bezier(0.33, 1, 0.68, 1);

		&:focus {
			border: 2px solid hsl(0, 0%, 92%);
			outline: none;
			background: hsl(0, 0%, 3%);
		}
	}

	input::-webkit-outer-spin-button,
	input::-webkit-inner-spin-button {
		-webkit-appearance: none;
		margin: 0;
	}
</style>
