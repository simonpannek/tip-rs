const colors = require('tailwindcss/colors');

/** @type {import('tailwindcss').Config} */
module.exports = {
	mode: 'jit',
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		colors: {
			transparent: 'transparent',
			current: 'currentColor',
			primary: {
				DEFAULT: '#ffffff',
				...colors.slate
			},
			secondary: { DEFAULT: colors.gray[600], ...colors.gray },
			accent1: { DEFAULT: colors.cyan[600], ...colors.cyan },
			accent2: { DEFAULT: colors.red[600], ...colors.red }
		},
		extend: {}
	},
	plugins: []
};
