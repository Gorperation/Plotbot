<script lang="ts">
	import Icon from './Icon.svelte'
</script>

<div>
	<svg
		width="32"
		height="45"
		viewBox="0 0 32 45"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
	>
		<path
			d="M22.7354 1.62424H30.6392V43.1191H22.7354V1.62424Z"
			fill="#161616"
		/>
		<path d="M1 1.62424H8.90378V43.1191H1V1.62424Z" fill="#161616" />
		<path
			d="M22.7354 1.62424H30.6392V43.1191H22.7354V1.62424Z"
			stroke="white"
			stroke-width="2"
		/>
		<path
			d="M1 1.62424H8.90378V43.1191H1V1.62424Z"
			stroke="white"
			stroke-width="2"
		/>
	</svg>
	<Icon />
	<svg
		width="61"
		height="61"
		viewBox="0 0 61 61"
		fill="none"
		xmlns="http://www.w3.org/2000/svg"
	>
		<path
			d="M21.9583 9.73473L39.5096 9.73472L51.9203 22.1454L51.9203 39.6967L39.5096 52.1074L21.9583 52.1074L9.54764 39.6967L9.54764 22.1454L21.9583 9.73473Z"
			fill="#161616"
			stroke="white"
			stroke-width="2"
		/>
	</svg>
</div>

<style lang="scss">
	div {
		display: flex;
		flex-direction: row;
		justify-content: center;
		align-items: center;
		gap: 2em;
	}
</style>
