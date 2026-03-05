const path = require("path");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const isProd = process.env.FLUSTER_PROD_BUILD === "true";

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js",
    publicPath: "./",
  },
  plugins: [
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  mode: isProd ? "production" : "development",
  experiments: {
    asyncWebAssembly: true,
  },
};
