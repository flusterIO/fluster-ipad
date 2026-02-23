import { Config } from "tailwindcss"
import containerQueries from "@tailwindcss/container-queries";

const config: Partial<Config> = {
    theme: {
        extend: {
            containers: {
                'mdxExpanded': '768px'
            }
        }
    },
    plugins: [
        containerQueries
    ]
}


export default config
