import {
    getBenchmarkContent,
    standardBenchmarkLengths,
} from "./get_fake_benchmark_content";
import path from "path";
import fs from "fs";

const outputDir = path.resolve(
    __dirname,
    "../../docs/development/generated_benchmark_content/",
);

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

writeBenchmarkData();
