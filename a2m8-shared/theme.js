const themes = require("daisyui/src/colors/themes");

module.exports = {
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