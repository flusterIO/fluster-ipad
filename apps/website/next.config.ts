import type { NextConfig } from "next";
import { createMDX } from "fumadocs-mdx/next";

const withMDX = createMDX({
    configPath: "./source.config.ts",
});

const nextConfig: NextConfig = {
    typescript: {
        ignoreBuildErrors: true,
    },
    reactStrictMode: true,
    images: {
        remotePatterns: [
            {
                protocol: "https",
                hostname: "ned.ipac.caltech.edu",
                pathname: "/**",
            },
        ],
        minimumCacheTTL: 3600,
    },
};

export default withMDX(nextConfig);
