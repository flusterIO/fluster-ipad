import fs from "fs";
import * as readline from "readline";
import path from "path";

const outputPath = path.resolve(
    __dirname,
    "../../docs/generated/benchmark_data.json",
);

// Set up an interface to read from standard input
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false,
});

console.log("Listening for Criterion benchmark data...");

rl.on("line", (line) => {
    // Cargo might print build status (non-JSON) before the benchmark runs,
    // so we wrap the parser in a try-catch to safely ignore compilation noise.
    try {
        const data = JSON.parse(line);
        data.testedOn = new Date().valueOf();
        // Criterion outputs various "reasons". We usually care about the final results.
        if (data.reason === "benchmark-complete") {
            const benchName = data.id;
            // The estimate is usually in nanoseconds
            const meanTimeNs = data.mean.estimate;

            console.log(`\n✅ Benchmark: ${benchName}`);
            console.log(`   Mean time: ${meanTimeNs.toFixed(2)} ns`);

            const fileData = JSON.parse(
                fs.readFileSync(outputPath, { encoding: "utf-8" }),
            ) as (typeof data)[];

            fileData.push(data);

            fs.writeFileSync(outputPath, JSON.stringify(fileData, null, 2), {
                encoding: "utf-8",
            });

            // Add your custom logic here (e.g., save to DB, fail CI if too slow)
        }
    } catch (error) {
        // Ignore lines that aren't valid JSON (like standard Cargo build output)
    }
});

rl.on("close", () => {
    console.log("Finished processing benchmark data.");
});
