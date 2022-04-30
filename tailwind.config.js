const colors = require('tailwindcss/colors');

module.exports = {
  content: [
    "./src/view/*.rs"
  ],
  prefix: 'tw-',
  darkMode: 'class',
  theme: {
    extend: {},
    colors: {
      current: 'currentColor',
      transparent: 'transparent',
      primary: {
        500: '#0078d4',
        800: '#0063b1',
      },
      gray: colors.gray,
      black: colors.black,
      white: colors.white,
      yellow: colors.amber,
      red: colors.red
    },
    textColor: {
      black: colors.black,
      blue: colors.blue,
      gray: {
        50: colors.gray[50],
      },
    }
  },
  plugins: [],
}
