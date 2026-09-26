/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        "bq-bg": {
          DEFAULT: "var(--color-bg-primary)",
          secondary: "var(--color-bg-secondary)",
          tertiary: "var(--color-bg-tertiary)",
        },
        "bq-accent": {
          DEFAULT: "var(--color-accent)",
          hover: "var(--color-accent-hover)",
          dim: "var(--color-accent-dim)",
        },
        "bq-text": {
          DEFAULT: "var(--color-text-primary)",
          secondary: "var(--color-text-secondary)",
          muted: "var(--color-text-muted)",
        },
        "bq-border": {
          DEFAULT: "var(--color-border)",
          active: "var(--color-border-active)",
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
