import {
    getBenchmarkContent,
    standardBenchmarkLengths,
} from "../../packages/webview_utils/src/development/fake_content_generators/get_benchmark_content";
import path from "path";
import fs from "fs";

const outputDir = path.resolve(
    __dirname,
    "../../docs/development/generated_benchmark_content/",
);

/**
 * Disabling function call at bottom of file and not adding to justfile to avoid overwriting existing benchmark data and muddying up the data.
 * Come back and write more content when more components have been added.
 */
const writeBenchmarkData = async () => {
    for await (const k of standardBenchmarkLengths) {
        const content = await getBenchmarkContent(k);
        const d = new Date();
        const outputPath = path.resolve(
            outputDir,
            `benchmark_${d.getMonth()}_${d.getDate()}_${d.getUTCFullYear()}_${k}.mdx`,
        );
        console.log("outputPath: ", outputPath);
        fs.writeFileSync(outputPath, content, {
            encoding: "utf-8",
        });
    }
};

// writeBenchmarkData();
