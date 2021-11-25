<script lang="ts">
	import { quadOut } from 'svelte/easing'

	import type { Toast } from './toast'

	export let toast: Toast
	const { message, type, timeout } = toast

	const fade = (node, { duration }) => ({
		duration,
		css: (t) => {
			const eased = quadOut(t)
			return `
					transform: translateY(${1 - eased}em);
					opacity: ${eased};`
		},
	})
</script>

<div class="toast {type}" transition:fade={{ duration: 200 }}>
	{message}
</div>

<style lang="scss">
	.toast {
		width: 16em;
		padding: 0.5em;
	}

	.success {
		background-color: hsl(122, 55%, 12%);
		border: 1px solid hsl(130, 30%, 35%);
	}
	.error {
		background-color: hsl(0, 58%, 12%);
		border: 1px solid hsl(357, 44%, 46%);
	}
	.info {
		background-color: hsl(200, 0%, 12%);
		border: 1px solid hsl(200, 0%, 45%);
	}
</style>
