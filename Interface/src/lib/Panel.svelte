<script lang="ts">
</script>

<div id="container">
	<div id="panel" />
	<!-- <div class="border" style="--m: 0.5em; --o: 0.8;" /> -->
	<!-- <div class="border" style="--m: 1.0em; --o: 0.6;" /> -->
	<h1><span>STATUS</span></h1>
</div>

<style lang="scss">
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

		.border {
			margin: var(--m);
			opacity: var(--o);
			height: calc(90% - var(--m) * 2);
			width: calc(100% - var(--m) * 2);
		}
	}
</style>
