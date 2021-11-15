<script lang="ts">
	import Drawings from './Drawings.svelte'
	import Status from './Status.svelte'
	import StopButtons from './StopButtons.svelte'
</script>

<div id="container">
	<div id="panel" class="specialBG" />
	<div id="content">
		<StopButtons />
		<Status />
		<Drawings />
	</div>
	<h1><span>STATUS</span></h1>
</div>

<style lang="scss">
	#content {
		position: relative;
		display: flex;
		flex-direction: column;
		gap: 2em;
		height: 90%;
		width: 85%;
		top: 4.8vh;
	}

	#container {
		width: 28%;
		max-width: 27em;
		height: 100%;
		margin-left: 2.5em;
		position: relative;
		display: inline-flex;
		flex-direction: row;
		justify-content: center;

		&:hover {
			span {
				background: rgb(1, 1, 1);
			}
		}

		h1 {
			position: absolute;
			width: 100%;
			/* left: -3.9em; */
			top: 0.1em;
			display: flex;
			flex-direction: row;
			justify-content: center;
			color: rgb(238, 238, 238);
			font-size: 1.8em;

			span {
				padding: 0em 0.35em;
				margin-top: 5vh;
				top: -1.4em;
				position: relative;
				text-align: center;
				background-color: black;
			}
		}

		#panel {
			position: absolute;
			border: 1px solid #b3b3b3;
			top: 5%;
			height: 90%;
			width: 100%;
		}

		#panel {
			/* background-color: black; */
			box-shadow: 6px 6px 0px -0.5px #000000, 6px 6px 0px 0.3px #fff;
			transition: all 0.35s;

			&:hover {
				background-color: hsl(0, 0%, 1%);
				box-shadow: 4px 4px 0px -0.5px #000000, 4px 4px 0px 0.3px #fff;
			}
		}
	}
</style>
