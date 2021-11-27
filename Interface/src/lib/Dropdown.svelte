<script lang="ts">
	export let options: { [display: string]: string } = { Dropdown: 'dropdown' }
	export let selected = 0
	export let value = Object.values(options)[selected]

	$: keys = Object.keys(options)
	$: value = Object.values(options)[selected]

	// const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms))

	let container: HTMLDivElement

	function select(event: MouseEvent, index: number) {
		selected = index
		container.blur()
	}

	function blur(event: MouseEvent) {
		if (!container.classList.contains('focus')) return
		event.preventDefault()
		container.blur()
	}
</script>

<div
	class="container {'focus' && 0}"
	tabindex="0"
	style="--num: {keys.length}"
	bind:this={container}
	on:focus={() => container.classList.add('focus')}
	on:blur={() => setTimeout(() => container.classList.remove('focus'), 100)}
>
	<svg
		width={9 * 1.2}
		height={6 * 1.2}
		viewBox="0 0 9 6"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
	>
		<path d="M1 1L4.5 4.5L8 1" stroke="white" />
	</svg>

	<option class="selected" on:mousedown={blur} value={options[keys[selected]]}
		>{keys[selected]}</option
	>
	<div class="list">
		{#each keys as key, i}
			<option on:click={(e) => select(e, i)} value={options[key]}
				>{key}</option
			>
		{/each}
	</div>
</div>

<style lang="scss">
	svg {
		position: absolute;
		top: 0.55em;
		right: 0.84em;
		transition: transform 0.2s ease-out;
	}

	option {
		padding: 0.12em 0.35em;
		user-select: none;

		font-family: 'Roboto', sans-serif;
		font-weight: 500;
		font-size: 0.91em;
		line-height: 1.3em;
		letter-spacing: 0.07em;

		background: black;

		&:not(.selected) {
			height: 1ch;
			display: none;
			transition: all 0.07s;

			&:hover {
				background-color: hsl(0deg, 0%, 94%);
				color: black;
			}
		}
	}

	.container {
		$padheight: 0.13em;
		padding: $padheight 0;
		padding-top: $padheight;
		/* background: $input-background; */
		border: 2px solid hsl(0, 0%, 92%);
		box-sizing: border-box;
		transition: all 0.2s cubic-bezier(0.33, 1, 0.68, 1);
		height: 1.9em;
		width: 10em;
		font-size: 0.9em;
		overflow: hidden;
		text-overflow: ellipsis;
		position: relative;
		z-index: 10;
		overflow: visible;

		display: inline-block;
		cursor: default;

		&:hover svg {
			transform: translateY(0.08em);
		}
		&.focus svg {
			transform: translateY(0.13em);
		}

		&:focus {
			height: calc(1.94em + 1.3em * var(--num));
			border-color: hsl(0, 0%, 99%);
			/* box-shadow: rgba(0, 0, 0, 0.1) 0px 4px 12px; */
		}
		&.focus {
			option {
				display: block;
			}
		}
	}
</style>
