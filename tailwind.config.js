/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{html,rs}', 'index.html'],

	theme: {
		container: {
			center: true,
			padding: {
				DEFAULT: '1rem', sm: '2rem', lg: '4rem', xl: '8rem', '2xl': '16rem',
			},

		},
	},
	plugins: [
		require('@tailwindcss/forms'),
		require('@tailwindcss/typography'),
		require("daisyui"),
	],
	daisyui: {
		themes: [
			{
				mytheme: {
					"primary": "#facc15",
					"secondary": "#3472cf",
					"accent": "#0064ff",
					"neutral": "#0f0f0f",
					"base-100": "#222222",
					"info": "#00adcb",
					"success": "#548100",
					"warning": "#ea7e00",
					"error": "#e6364a",
				},
			},
		],
	},
};
