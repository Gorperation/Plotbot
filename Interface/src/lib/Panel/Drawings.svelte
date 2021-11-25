<script lang="ts">
	import { Status } from './index.d'
	import Drawing from './Drawing.svelte'
	import Label from './Label.svelte'
	import { drawings } from 'src/storage'
</script>

<div class="container">
	<Label>DRAWINGS</Label>
	{#each $drawings as drawing}
		<Drawing {...drawing} />
	{/each}
</div>

<style lang="scss">
</style>
