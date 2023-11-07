module.exports = {
  mode: "jit",
  content: {
    files: ["src/**/*.rs", "**/*.html"],
  },
  darkMode: "media", // 'media' or 'class'
  theme: {
    extend: {},
  },
  variants: {
    extend: {
      backgroundImage: ['hover', 'focus'],
    },
  },
  plugins: [require("daisyui")],
};
