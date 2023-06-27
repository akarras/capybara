
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      aria: {
        current: 'current'
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
