/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",
  content: ["*.html", "./app/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [
    require("@catppuccin/tailwindcss")({
      prefix: "ctp",
      defaultFlavour: "latte",
    }),
  ],
};
