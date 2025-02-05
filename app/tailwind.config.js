/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            colors: {
                primary: '#c45e2e',    // Warm orange accent from waypoint
                surface: {
                    DEFAULT: '#2a2a2f', // Card/component background
                    dark: '#1a1a1d'     // Deep background
                },
                text: {
                    DEFAULT: '#e0d5c7', // Antique white text
                    muted: '#8b8178'    // Muted text
                },
                border: '#3d3d45'       // Subtle borders
            }
        }
    },
    plugins: []
} 