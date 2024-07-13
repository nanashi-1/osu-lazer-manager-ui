import flowbitePlugin from 'flowbite/plugin'

import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}', './node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
      colors: {
        primary: {
          50: '#fce5f4',
          100: '#f6bee3',
          200: '#ee92d1',
          300: '#e364bd',
          400: '#d93cae',
          500: '#ce00a1',
          600: '#c0009c',
          700: '#ae0096',
          800: '#9e0090',
          900: '#810085'
        }
      }
    }
	},

	plugins: [flowbitePlugin]
} as Config;
