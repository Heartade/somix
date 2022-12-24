/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["index.html", "./src/**/*.rs"],
  theme: {
    extend: {},
    colors: {
      'charm': {
        50:  '#f2fdfb',
        100: '#e6faf7',
        200: '#bff4ec',
        300: '#99ede0',
        400: '#4ddfc9',
        500: '#00d1b2',
        600: '#00bca0',
        700: '#009d86',
        800: '#007d6b',
        900: '#006657',
      },
      'tuatara': {
        50:  '#f5f5f5',
        100: '#ebebeb',
        200: '#cdcdcd',
        300: '#afafaf',
        400: '#727272',
        500: '#363636',
        600: '#313131',
        700: '#292929',
        800: '#202020',
        900: '#1a1a1a',
      },
      'stiletto': {
        50:  '#faf5f5',
        100: '#f5ebeb',
        200: '#e6cdcc',
        300: '#d7afae',
        400: '#ba7471',
        500: '#9C3834',
        600: '#8c322f',
        700: '#752a27',
        800: '#5e221f',
        900: '#4c1b19',
      }
    },
  },
  plugins: [],
}
