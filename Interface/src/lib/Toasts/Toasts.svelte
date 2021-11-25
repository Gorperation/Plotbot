<script lang="ts">
	import Toast from './Toast.svelte'
	import { toastStack } from './toast'
</script>

<div class="toasts">
	{#each $toastStack as toast}
		<Toast {toast} />
	{/each}
</div>

<style lang="scss">
	.toasts {
		position: fixed;
		bottom: 2em;
		left: 2em;
		display: flex;
		flex-direction: column;
		gap: 0.6em;
	}
</style>
