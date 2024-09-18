/** @type {import('tailwindcss').Config} */
export default {
  darkMode: "class",
  content: { 
    files: ["./site/*.html", "./site/src/**/*.rs", "./crates/**/*.rs"],
  },
  theme: {
    extend: {
      fontFamily: {
        roboto: ["Roboto", "sans-serif"],
      },
      colors: {
        primary: "#DC5F00",
        btn: "#334155",
        secondary: "#0D0F10",
      },
      extend: {
        fontFamily: {
          roboto: ["Roboto", "sans-serif"],
        }
      }
    },
  },
  plugins: [],
};
