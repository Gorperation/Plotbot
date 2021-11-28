<script lang="ts">
	import { slide } from 'src/ts/animations'
	import { draggable } from 'svelte-drag'
	import { onmessage, sendMessage } from 'src/ts/websockets'

	let shown = false
	let stream: string[] = []
	let history: string[] = []
	let historyIndex = -1
	let input: HTMLInputElement

	onmessage((event: MessageEvent) => {
		const { data } = event
		if (typeof data === 'string') {
			if (data.startsWith('PING:')) return
			stream = [...stream, data]
		} else {
			const decoder = new TextDecoder()
			const text = decoder.decode(data)
			stream = [...stream, text]
		}
	})

	document.addEventListener('keydown', (event) => {
		if (event.key == 't' && document.activeElement == document.body)
			shown = !shown

		if (event.key == 'k' && event.ctrlKey) {
			event.preventDefault()
			stream = []
		}
	})

	function keyDown(event: KeyboardEvent) {
		switch (event.key) {
			case 'Enter': {
				const command = input.value.trim()
				stream = [...stream, '> ' + command]
				history.unshift(command)
				input.value = ''
				historyIndex = -1
				sendMessage(command + '\n')
				break
			}

			case 'ArrowUp': {
				event.preventDefault()
				if (historyIndex < history.length) historyIndex++
				else break
				const command = history[historyIndex]
				if (command) input.value = command
				break
			}

			case 'ArrowDown': {
				if (historyIndex > 0) historyIndex--
				else {
					input.value = ''
					break
				}
				const command = history[historyIndex]
				if (command) input.value = command
				break
			}
		}
	}
</script>

{#if shown}
	<div
		use:draggable={{ bounds: 'parent' }}
		transition:slide={{ duration: 200 }}
		class="container"
	>
		<div class="content">
			{#each stream as item}
				<div class="item">{item}</div>
			{/each}
		</div>
		<div class="text-bar">
			<input bind:this={input} on:keydown={keyDown} type="text" />
		</div>
	</div>
{/if}

<style lang="scss">
	.container {
		position: absolute;
		z-index: 11;
		right: 2em;
		bottom: 2em;
		width: 30vw;
		height: 40vh;
		border: 1px solid #b3b3b3;
		background-color: black;
		font-family: 'Roboto Mono';
		font-size: 0.95em;

		.content {
			height: calc(100% - 4.3em);
			padding: 0.8em;
			padding-left: 1.1em;
			overflow-y: scroll;

			&::-webkit-scrollbar {
				display: none;
			}

			.item {
				padding-left: 1em;
				text-indent: -1em;
			}
		}

		.text-bar {
			display: flex;
			flex-direction: row;
			justify-content: center;

			input {
				outline: none;
				padding: 0.3em 0.5em;
				width: 91%;
				border: none;
				border: 1px solid #b3b3b3;
				background-color: #090909;
				color: white;
				font-family: 'Roboto Mono';
				font-size: 0.9em;
			}
		}
	}
</style>
