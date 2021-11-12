<script lang="ts">
	export let options = ['Dropdown']
	export let selected = 0

	const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms))

	let container: HTMLDivElement

	function select(event: MouseEvent, index: number) {
		selected = index
		container.blur()
	}
</script>

<div
	class="container {'focus' && 0}"
	tabindex="0"
	style="--num: {options.length}"
	bind:this={container}
	on:focus={() => container.classList.add('focus')}
	on:blur={() => setTimeout(() => container.classList.remove('focus'), 100)}
>
	<svg
		width="9"
		height="6"
		viewBox="0 0 9 6"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
	>
		<path d="M1 1L4.5 4.5L8 1" stroke="black" />
	</svg>

	<option class="selected">{options[selected]}</option>
	<div class="list">
		{#each options as option, i}
			<option on:click={(e) => select(e, i)}>{option}</option>
		{/each}
	</div>
</div>

<style lang="scss">
	svg {
		cursor: pointer;
		position: absolute;
		top: 0.6em;
		right: 0.5em;
	}

	option {
		padding: 0.12em 0.35em;

		&:not(.selected) {
			height: 1ch;
			display: none;
			transition: all 0.07s;

			&:hover {
				background-color: hsl(0deg, 0%, 90%);
			}
		}
	}

	.container {
		$padheight: 0.13em;
		padding: $padheight 0;
		padding-top: $padheight;
		/* background: $input-background; */
		border: 2px solid #2b2b2b;
		box-sizing: border-box;
		border-radius: 5px;
		transition: all 0.2s cubic-bezier(0.33, 1, 0.68, 1);
		height: 1.9em;
		width: 7.5em;
		font-size: 0.9em;
		overflow: hidden;
		position: relative;

		display: inline-block;
		cursor: default;

		&:hover {
			box-shadow: rgba(0, 0, 0, 0.1) 0px 4px 12px;
		}

		&:focus {
			height: calc(1.9em + 1.43em * var(--num));
			box-shadow: rgba(0, 0, 0, 0.1) 0px 4px 12px;

			option:not(.selected) {
				/* cursor: pointer; */
			}
		}
		&:not(.focus) option {
			cursor: pointer;
		}
		&.focus {
			option {
				display: block;
			}
		}
	}
</style>
