<script lang="ts">
	import Icon from './Icon.svelte'
</script>

<div>
	<span>
		<svg
			width="42"
			height="42"
			viewBox="0 0 48 48"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
		>
			<rect
				width="12.5"
				height="48"
				fill="#161616"
				stroke="white"
				stroke-width="2"
				x="5"
			/>
			<rect
				width="12.5"
				height="48"
				fill="#161616"
				stroke="white"
				stroke-width="2"
				x="31"
			/>
		</svg>
	</span>
	<!-- <Icon /> -->
	<span>
		<svg
			width="42"
			height="42"
			viewBox="0 0 48 48"
			fill="none"
			xmlns="http://www.w3.org/2000/svg"
		>
			<path
				d="M14.935 1.856L33.715 1.856L46.995 15.136L46.995 33.915L33.715 47.195L14.935 47.195L1.656 33.915L1.656 15.136L14.935 1.856Zh2.14"
				fill="#161616"
				stroke="white"
				stroke-width="2"
			/>
		</svg>
	</span>
</div>

<style lang="scss">
	div {
		margin-top: 2.5em;
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		gap: 3.4em;
	}

	span {
		cursor: pointer;
		transition: all 0.2s;

		&:hover {
			transform: scale(1.03);
		}
	}
</style>
