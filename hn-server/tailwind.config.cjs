const contentFileFilter = __dirname + "/**/*.{rs,html.j2}";
console.log({ contentFileFilter });
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [contentFileFilter],
  theme: {
    extend: {},
  },
  plugins: [],
};
