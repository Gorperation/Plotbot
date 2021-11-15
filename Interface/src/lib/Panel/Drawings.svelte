<script lang="ts">
	import { Status } from './index.d'
	import Drawing from './Drawing.svelte'
	import Label from './Label.svelte'
</script>

<div class="container">
	<Label>DRAWINGS</Label>
	<Drawing
		name={'Drawing.svg'}
		status={Status.Drawing}
		sizeKB={1200}
		time={123}
	/>
	<!-- <Drawing /> -->
	<!-- <Drawing /> -->
	<!-- <Drawing /> -->
</div>

<style lang="scss">
</style>
