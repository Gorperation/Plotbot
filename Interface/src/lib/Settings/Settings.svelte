<script lang="ts">
	import Field from './../Field.svelte'
	import Checkbox from './../Checkbox.svelte'
	import Dropdown from './../Dropdown.svelte'
	import Button from './../Button.svelte'
	import Label from './Label.svelte'

	let fills = {
		Grid: 'monotonic',
		Triangles: 'triangles',
		Lattice: 'adaptivecubic',
		Stars: 'stars',
		Hexagons: 'honeycomb',
		Chain: 'hexagonal',
		Waves: 'gyroid',
		Maze: 'hilbertcurve',
		Spiral: 'archimedeanchords',
		Octagram: 'octagramspiral',
		Scattered: 'scatteredrectilinear',
	}
</script>

<div id="container" class="specialBG">
	<h1>Settings</h1>
	<div class="settings">
		<Label text="Fill"><Checkbox /></Label>
		<div class="fill-options">
			<Label text="Density" unit="%">
				<Field value="100" type="number" size="2" length={3} />
			</Label>
			<Label text="Pattern"><Dropdown options={fills} /></Label>
			<Label text="Connected"><Checkbox /></Label>
			<Label text="Overlap" unit="%">
				<Field
					value="100"
					type="number"
					size="2"
					max={100}
					length={3}
				/>
			</Label>
			<Label text="Angle">
				<Field value="45" type="number" size="2" length={3} />
			</Label>
			<Label text="Density" unit="%">
				<Field value="100" size="2" length={3} />
			</Label>
		</div>
	</div>
</div>

<style lang="scss">
	#container {
		border: 1px solid white;
		height: 60%;
		width: 15vw;
		padding: 1em 1.56em;
		position: absolute;
		right: calc(24vw - 20em);
		top: 19vh;
	}

	.fill-options {
		opacity: 0.8;
	}

	h1 {
		margin: 0;
		margin-bottom: 0.4em;
	}
</style>
