/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: 'jit',
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  darkMode: "class",
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
}
