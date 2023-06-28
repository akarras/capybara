
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
      },
      visibility: 'group-hover'
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
