<script lang="ts">
	let paused = false
</script>

<div id="scroller" class:pause={paused} on:click={() => (paused = !paused)}>
	<div>
		<p>
			PLOTBOT • DRAWING INNOVATION • ACCURACY • AI • PLOTBOT •
			COMPUTATIONAL PICTOGRAPHY • ML • PRECISION • CNC • PLOTBOT • GCODE •
			PLOTBOT • MODERN-ART • DEDICATED
		</p>
		<p>
			PLOTBOT • DRAWING INNOVATION • ACCURACY • AI • PLOTBOT •
			COMPUTATIONAL PICTOGRAPHY • ML • PRECISION • CNC • PLOTBOT • GCODE •
			PLOTBOT • MODERN-ART • MACHINE •
		</p>
	</div>
</div>

<style lang="scss">
	#scroller {
		position: absolute;
		top: 0;
		width: 100%;
		height: 2em;
		line-height: 2em;
		font-size: 1em;
		background: #f5f5f5;
		cursor: default;

		&.pause div {
			animation-play-state: paused;
		}

		@keyframes scroll {
			from {
				transform: translateX(-50%);
			}
			to {
				transform: translateX(0%);
			}
		}

		div {
			display: inline-block;
			color: black;
			overflow: hidden;
			text-overflow: clip;
			font-weight: bold;
			white-space: nowrap;
			animation: scroll 105s forwards infinite linear;

			p {
				margin: 0;
				display: inline-block;
			}
		}
	}
</style>
