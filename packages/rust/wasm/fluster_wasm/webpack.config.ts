import path from "path";
import webpack from "webpack";
import type { Configuration as WebpackConfiguration } from "webpack";
import type { Configuration as WebpackDevServerConfiguration } from "webpack-dev-server";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

interface Configuration extends WebpackConfiguration {
    devServer?: WebpackDevServerConfiguration;
}

const isProd = process.env.FLUSTER_PROD_BUILD === "true";

const config: Configuration = {
    mode: isProd ? "production" : "development",
    entry: {
        main: "./index.js",
    },
    output: {
        path: path.resolve(import.meta.dirname, "dist"),
        filename: "[name].bundle.js",
        publicPath: "./",
        clean: true,
    },
    resolve: {
        extensions: [".ts", ".tsx", ".js", ".jsx", ".wasm"],
        alias: {
            // Mapping your monorepo aliases
            "@": path.resolve(import.meta.dirname, "src/core"),
            "#": path.resolve(import.meta.dirname, "src/features"),
            path: "path-browserify",
        },
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: "webassembly/async",
            },
        ],
    },
    // module: {
    //     rules: [
    //         {
    //             test: /\.(ts|tsx)$/,
    //             loader: "ts-loader",
    //             exclude: /node_modules/,
    //         },
    //         {
    //             test: /\.css$/,
    //             use: [
    //                 "style-loader",
    //                 "css-loader",
    //                 // PostCSS is required for Tailwind in Webpack
    //                 {
    //                     loader: "postcss-loader",
    //                     options: {
    //                         postcssOptions: {
    //                             plugins: ["@tailwindcss/postcss", "autoprefixer"],
    //                         },
    //                     },
    //                 },
    //             ],
    //         },
    //     ],
    // },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: import.meta.dirname,
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
        topLevelAwait: true,
    },
    devtool: isProd ? "source-map" : "eval-source-map",
};

export default config;
