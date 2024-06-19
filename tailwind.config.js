/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "app/**/*.rs"],
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
