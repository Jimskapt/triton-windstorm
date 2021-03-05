const colors = require('tailwindcss/colors');

module.exports = {
  purge: [
    "./src/view/*.rs"
  ],
  prefix: 'tw-',
  darkMode: 'class',
  theme: {
    extend: {},
    colors: {
      transparent: 'transparent',
      primary: {
        500: '#0078d4',
        800: '#106ebe',
      },
      gray: colors.coolGray,
      black: colors.black,
      white: colors.white,
    },
    textColor: {
      black: colors.black,
      gray: {
        50: colors.coolGray[50],
      },
    }
  },
  variants: {
    extend: {
      borderWidth: ['hover'],
      ringWidth: ['active'],
    },
  },
  plugins: [],
}
