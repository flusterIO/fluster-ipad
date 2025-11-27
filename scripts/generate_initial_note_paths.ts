import path from "path";
import fs from "fs";

const initialNotesDir = path.resolve(
    import.meta.dirname ?? __dirname,
    "../docs/initial_note_docs",
);

const paths = fs
    .readdirSync(initialNotesDir, {
        encoding: "utf-8",
    })
    .filter((_path) => _path !== "initial_note_paths.json")
    .map((d) => `initial_note_docs/${d.slice(0, d.lastIndexOf("."))}`);

fs.writeFileSync(
    path.join(initialNotesDir, "initial_note_paths.json"),
    JSON.stringify(paths, null, 2),
    {
        encoding: "utf-8",
    },
);

console.log(`Successfully wrote initial note file paths to file.`);
