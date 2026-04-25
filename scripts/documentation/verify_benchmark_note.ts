import fs from "fs";
import path from "path";

const changeLogFilePath = path.resolve(
    __dirname,
    "../../docs/development/script_inputs/benchmark_changelog.md",
);

const content = fs.readFileSync(changeLogFilePath, { encoding: "utf-8" });

if (!content.trim()) {
    console.log(
        "Cannot run benchmarks without a benchmark log in the benchmark_changelog.md file because I'm too lazy to set this up properly.",
    );
    process.exit(1);
}
