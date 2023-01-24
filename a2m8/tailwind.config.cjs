const daisyui = require("daisyui");
const themes = require("daisyui/src/colors/themes");

const basic = {
  ...themes["[data-theme=business]"],
  ...themes["[data-theme=forest]"],
  neutral: themes["[data-theme=black]"].neutral,
  accent: themes["[data-theme=black]"].accent,
  secondary: themes["[data-theme=acid]"].secondary,
  ["base-100"]: themes["[data-theme=halloween]"]["base-100"],
  ["base-200"]: themes["[data-theme=halloween]"]["base-200"],
  ["base-300"]: themes["[data-theme=black]"]["base-300"],
  "--rounded-btn": "0.4rem",
};

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {},
  },
  plugins: [daisyui],
  daisyui: {
    logs: false,
    themes: [{ basic }],
  },
};
