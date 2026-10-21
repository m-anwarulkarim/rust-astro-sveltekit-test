/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        gold: {
          500: '#D4AF37',
          600: '#B89020',
        }
      }
    },
  },
  plugins: [],
}
