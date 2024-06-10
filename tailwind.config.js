/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "selector",
  content: ["*.html", "./app/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [
    require("@tailwindcss/forms"),
    require("@tailwindcss/typography"),
    require("@catppuccin/tailwindcss"),
  ],
};
