<script lang="ts">
	import { options as optionStore } from 'src/storage'
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
	let perimeter = true
	$: options = {
		fill_density: fill ? '100' : '0',
		fill_pattern: 'monotonic',
		fill_connected: true,
		fill_overlap: '100',
		fill_angle: '45',
		fill_speed: '60',
		perimeters: '1',
		perimeter_speed: '60',
	}

	$: {
		optionStore.set({
			fill_density: parseInt(options.fill_density),
			fill_pattern: options.fill_pattern,
			fill_connected: options.fill_connected,
			fill_overlap: parseInt(options.fill_overlap),
			fill_angle: parseInt(options.fill_angle),
			fill_speed: parseInt(options.fill_speed),
			perimeters: parseInt(options.perimeters),
			perimeter_speed: parseInt(options.perimeter_speed),
		})
	}
</script>

<div class="container">
	<h1>Settings</h1>
	<div class="settings">
		<Label text="Fill"><Checkbox bind:value={fill} /></Label>
		<div class="subcategory {fill && 'shown'}">
			<Label text="Density" unit="%">
				<Field
					type="number"
					size="2"
					length={3}
					bind:value={options.fill_density}
				/>
			</Label>
			<Label text="Pattern" top>
				<Dropdown options={fills} bind:value={options.fill_pattern} />
			</Label>
			<Label text="Connected">
				<Checkbox bind:value={options.fill_connected} />
			</Label>
			<Label text="Overlap" unit="%">
				<Field
					type="number"
					size="2"
					max={100}
					length={3}
					bind:value={options.fill_overlap}
				/>
			</Label>
			<Label text="Angle" unit="º">
				<Field
					type="number"
					size="2"
					length={3}
					bind:value={options.fill_angle}
				/>
			</Label>
		</div>
		<Label text="Outline">
			<Checkbox bind:value={perimeter} />
		</Label>
		<div class="subcategory {perimeter && 'shown'}">
			<Label text="Repetitions">
				<Field size="2" length={3} bind:value={options.perimeters} />
			</Label>
			<Label text="Speed">
				<Field
					size="2"
					length={3}
					bind:value={options.perimeter_speed}
				/>
			</Label>
		</div>
	</div>
</div>

<style lang="scss">
	.container {
		/* border: 1px solid white; */
		height: 60%;
		padding: 1em 1.56em;
		/* padding-left: 5em; */
		display: inline-block;
		position: relative;
		margin-right: 5em;
		margin-top: 3em;
		/* flex: 1; */
		flex-shrink: 1;
		align-self: flex-start;
	}

	.subcategory {
		margin-bottom: 0.5em;

		// cascade effect
		& > :global(.container) {
			opacity: 0;
			height: 0em;
			transform: translateY(-0.3em) translateX(0.15em);
		}

		&.shown > :global(.container) {
			opacity: 1;
			height: 2em;
			transform: translateY(0em) translateX(0.5em);
		}

		@for $i from 1 through 10 {
			& > :global(:nth-child(#{$i})) {
				transition: all 0.1s cubic-bezier(0.33, 1, 0.68, 1);
				transition-delay: #{(10-$i) * 0.02}s;
			}
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
