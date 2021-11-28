import { quadOut } from 'svelte/easing'

export const slide = (
	node,
	{ duration = 300, distance = 1, axis = 'Y', easing = quadOut }
) => ({
	duration,
	css: (t) => {
		const eased = easing(t)
		return `
				transform: translate${axis}(${(1 - eased) * distance}em);
				opacity: ${eased};`
	},
})
