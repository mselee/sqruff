/** @type {import('tailwindcss').Config} */
const defaultTheme = require("tailwindcss/defaultTheme");

module.exports = {
  darkMode: "class",
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        "ayu-accent": "#ffac2f",
        "ayu-background": {
          DEFAULT: "#f8f9fa",
          dark: "#0b0e14",
        },
        black: "#261230",
        white: "#FFFFFF",
        radiate: "#D7FF64",
        flare: "#6340AC",
        rock: "#78876E",
        galaxy: "#261230",
        space: "#30173D",
        comet: "#6F5D6F",
        cosmic: "#DE5FE9",
        sun: "#FFAC2F",
        electron: "#46EBE1",
        aurora: "#46EB74",
        constellation: "#5F6DE9",
        neutron: "#CFF3CF",
        proton: "#F6AFBC",
        nebula: "#CDCBFB",
        supernova: "#F1AFF6",
        starlight: "#F4F4F1",
        lunar: "#FBF2FC",
        asteroid: "#E3CEE3",
        crater: "#F0DFDF",
      },
      fontFamily: {
        heading: [],
        body: [],
        mono: ["Roboto Mono"],
      },
      fontSize: {
        xs: "0.75rem" /* 12px */,
        sm: "0.875rem" /* 14px */,
        base: "1rem" /* 16px */,
        lg: "1.125rem" /* 18px */,
        xl: "1.25rem" /* 20px */,
        "2xl": "1.5rem" /* 25px */,
        "3xl": "1.875rem" /* 30px */,
        "4xl": "2.25rem" /* 36px */,
        "5xl": "3rem" /* 48px */,
      },
    },
  },
  plugins: [],
};