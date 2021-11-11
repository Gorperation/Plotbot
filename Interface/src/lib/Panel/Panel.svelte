<script lang="ts">
	import StopButtons from './StopButtons.svelte'
</script>

<div id="container">
	<div id="panel" />
	<div id="content">
		<StopButtons />
	</div>
	<h1><span>STATUS</span></h1>
</div>

<style lang="scss">
	#content {
		position: relative;
	}

	#container {
		width: 28%;
		height: 100%;
		margin-left: 2.5em;
		position: relative;
		display: inline-block;

		&:hover {
			span {
				background: rgb(1, 1, 1);
			}
		}

		h1 {
			position: absolute;
			width: 100%;
			/* left: -3.9em; */
			display: flex;
			flex-direction: row;
			justify-content: center;
			color: rgb(238, 238, 238);
			font-size: 1.8em;

			span {
				padding: 0em 0.35em;
				text-align: center;
				background-color: black;
			}
		}

		div {
			position: absolute;
			border: 1px solid #b3b3b3;
			top: 5%;
			height: 90%;
			width: 100%;
		}

		#panel {
			background-color: black;
			box-shadow: 6px 6px 0px -0.5px #000000, 6px 6px 0px 0.5px #b3b3b3;
			transition: all 0.35s;

			&:hover {
				background: rgb(1, 1, 1);
				box-shadow: 5px 5px 0px -0.5px #000000,
					5px 5px 0px 0.5px #b3b3b3;
			}
		}
	}
</style>
