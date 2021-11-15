<script lang="ts">
	import ProgressBar from './ProgressBar.svelte'
	import Label from './Label.svelte'
</script>

<div>
	<Label>STATUS</Label>
	<p>Plotting your art...</p>
	<p>About <span class="time">1 hour 10 minutes</span> remaining</p>
	<ProgressBar />
</div>

<style lang="scss">
	P {
		margin: 0.1em 0;
		color: #f3f3f3;
	}

	.time {
		color: #ffc700;
	}

	p:nth-of-type(2) {
		margin-bottom: 0.3em;
	}
</style>
