/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			fontFamily: {
				'kiwi-maru': ['Kiwi Maru'],
				'dot-gothic16': ['DotGothic16']
			},
			colors: {
				white: 'var(--white)',
				black: 'var(--black)',
				yellow: 'var(--yellow)',
				green: 'var(--green)',
				gray: 'var(--gray)'
			}
		}
	},
	plugins: []
};
