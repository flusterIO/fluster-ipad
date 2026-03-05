const path = require("path");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "..", "dist", "hello_world"),
        filename: "index.js",
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: __dirname,
        }),
    ],
    mode: "development",
    experiments: {
        asyncWebAssembly: true,
    },
};
