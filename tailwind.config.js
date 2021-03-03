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
      current: colors.coolGray,
      primary: {
        500: '#0078d4',
        800: '#106ebe',
      },
      gray: colors.coolGray,
      white: colors.white,
    },
    textColor: {
      primary: colors.white,
    }
  },
  variants: {
    extend: {
      borderWidth: ['hover'],
      // borderColor: ['active'],
    },
  },
  plugins: [],
}
