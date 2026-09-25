/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        // Biquad Studio brand palette
        "bq-bg": {
          DEFAULT: "#0a0a0f",
          secondary: "#12121a",
          tertiary: "#1a1a26",
        },
        "bq-accent": {
          DEFAULT: "#6366f1", // Indigo-500
          hover: "#818cf8",   // Indigo-400
          dim: "#4338ca",     // Indigo-700
        },
        "bq-text": {
          DEFAULT: "#e2e8f0",
          secondary: "#94a3b8",
          muted: "#64748b",
        },
        "bq-border": {
          DEFAULT: "#1e293b",
          active: "#334155",
        },
        "bq-meter": {
          green: "#22c55e",
          yellow: "#eab308",
          red: "#ef4444",
        },
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "-apple-system", "sans-serif"],
        mono: ["JetBrains Mono", "Fira Code", "monospace"],
      },
    },
  },
  plugins: [],
};
