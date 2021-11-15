<script lang="ts">
	import Field from '../Field.svelte'
	import Checkbox from '../Checkbox.svelte'
	import Dropdown from '../Dropdown.svelte'
	import Button from '../Button.svelte'
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

	let fill = false
	let options = {
		fill_density: fill ? '100' : '0',
		fill_pattern: '',
		fill_connected: true,
		fill_overlap: '100',
		fill_angle: '45',
		fill_speed: '60',
		perimeters: '1',
		perimeter_speed: '60',
	}
</script>

<div class="container blur">
	<h1>Settings</h1>
	<div class="settings">
		<Label text="Fill"><Checkbox bind:checked={fill} /></Label>
		<div class="fill-options {fill && 'shown'}">
			<Label text="Density" unit="%">
				<Field
					type="number"
					size="2"
					length={3}
					bind:value={options.fill_density}
				/>
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
	.container {
		border: 1px solid white;
		height: 60%;
		width: 15vw;
		padding: 1em 1.56em;
		position: absolute;
		right: calc(24vw - 20em);
		top: 19vh;
	}

	.fill-options {
		// cascade effect
		@for $i from 1 through 10 {
			& > :global(:nth-child(#{$i})) {
				transition: all 0.05s cubic-bezier(0.33, 1, 0.68, 1);
				transition-delay: #{(10-$i) * 0.02}s;
			}
		}

		& > :global(.container) {
			opacity: 0;
			transform: translateY(-0.3em) translateX(0.15em);
		}

		&.shown > :global(.container) {
			opacity: 1;
			transform: translateY(0em) translateX(0.5em);
		}

		&.shown {
			@for $i from 1 through 10 {
				& > :global(:nth-child(#{$i})) {
					transition: all 0.3s cubic-bezier(0.33, 1, 0.68, 1);
					transition-delay: #{$i * 0.03}s;
				}
			}
		}
	}

	h1 {
		margin: 0;
		margin-bottom: 0.4em;
	}
</style>
