/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "*.html",
    "./app/**/*.rs",
    "./server/**/*.rs",
    "./frontend/**/*.rs",
    "./src/**/*.rs",
  ],
  theme: {
    extend: {},
  },
  plugins: ["@catppuccin/tailwindcss"],
};
