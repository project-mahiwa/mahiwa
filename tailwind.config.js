/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
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
