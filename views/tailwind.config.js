const colors = require("tailwindcss/colors");

module.exports = {
  purge: ["templates/*.html.hbs"],
  darkMode: false, // or 'media' or 'class'
  theme: {
    colors: { ...colors, blue: colors.cyan },
    extend: {
      spacing: {
        full: "100%",
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [require("autoprefixer")],
};
