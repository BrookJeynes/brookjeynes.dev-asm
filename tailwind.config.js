/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {
            colors: {
                "black": "#1b1b1b",
                "green": "#a3be8c",
                "blue": "#7BB5E8"
            },
            animation: {
                "cursor-blink": "blink 1.5s steps(2) infinite"
            },
            keyframes: {
                "blink": {
                    "0%": { opacity: 0 },
                }
            }
        },
    },
    plugins: [],
}
