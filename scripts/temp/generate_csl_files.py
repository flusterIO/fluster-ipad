from pathlib import Path


embedded_csl_dir = Path(
    "/Users/bigsexy/Desktop/swift/Fluster/packages/rust/fluster_bibliography/src/data/embedded/csl/"
)

output_dir = embedded_csl_dir / "output" / "styles"


input_files = list(embedded_csl_dir.iterdir())

filtered_target_files = []

for f in input_files:
    if f"{f}".endswith(".csl"):
        filtered_target_files.append({"name": f.name, "path": f})


print(filtered_target_files[0])
source_files = []

for sf in output_dir.glob("*.csl"):
    name = sf.name
    matching_file = None
    for tf in filtered_target_files:
        if tf["name"] == name:
            matching_file = tf
    if matching_file:
        target_path = Path(matching_file["path"])
        source_path = Path(sf)
        target_path.write_text(source_path.read_text())

# print(list(sf))
