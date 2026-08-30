from pathlib import Path
import os

root = Path(os.environ["FLUSTER_IOS_ROOT"])

inputDir = root / "packages/rust/conundrum_docgen/templates/rust/db/"

outputDir = root / "packages/conundrum_db/src/vector/"

inputFiles = inputDir.glob("**/*")

for f in inputFiles:
    if f.is_file():
        trimmedPath = f.relative_to(inputDir)
        outputPath = outputDir / trimmedPath
        print(outputPath)
