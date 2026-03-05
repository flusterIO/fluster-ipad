import path from "path";
import webpack from "webpack";
import { Configuration as WebpackConfiguration } from "webpack";
import { Configuration as WebpackDevServerConfiguration } from "webpack-dev-server";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

interface Configuration extends WebpackConfiguration {
    devServer?: WebpackDevServerConfiguration;
}

const isProd = process.env.FLUSTER_PROD_BUILD === "true";

const config: Configuration = {
    mode: isProd ? "production" : "development",
    entry: {
        main: "./src/index.ts",
        wasm: "./src/wasm_index.ts",
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        // Essential for your Swift app:// scheme
        filename: "[name].bundle.js",
        publicPath: "./",
        clean: true,
    },
    resolve: {
        extensions: [".ts", ".tsx", ".js", ".jsx", ".wasm"],
        alias: {
            // Mapping your monorepo aliases
            "@": path.resolve(__dirname, "src/core"),
            "#": path.resolve(__dirname, "src/features"),
            path: "path-browserify",
        },
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
            crateDirectory: __dirname,
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
        topLevelAwait: true,
    },
    devtool: isProd ? "source-map" : "eval-source-map",
};

export default config;
