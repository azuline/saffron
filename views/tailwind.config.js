const colors = require("tailwindcss/colors");

module.exports = {
  content: ["templates/*.html"],
  theme: {
    colors: {
      black: colors.black,
      white: colors.white,
      gray: colors.slate,
      blue: colors.cyan,
    },
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
