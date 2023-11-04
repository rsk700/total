/** @type {import('tailwindcss').Config} */
const defaultTheme = require('tailwindcss/defaultTheme');

export default {
  content: ["./src/**/*.{html,js,svelte,ts}", "./index.html"],
  theme: {
    extend: {}
    ,
    fontFamily: {
      ...defaultTheme.fontFamily,
      'sans': ['Roboto', ...defaultTheme.fontFamily.sans],
    }
  },
  plugins: [],
}

