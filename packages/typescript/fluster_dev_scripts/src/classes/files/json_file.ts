import fs from "fs";
import { FileHelper } from "./file_helper";
import deepMerge from "ts-deepmerge";

export class JsonFile<T extends object> extends FileHelper<T> {
    constructor(filePath: string) {
        super(filePath);
    }
    read(): T {
        if (!fs.existsSync(this.path)) {
            throw new Error(`The ${this.path} file does not exist.`);
        }
        return JSON.parse(fs.readFileSync(this.path, { encoding: "utf-8" }));
    }

    write(content: T): void {
        fs.writeFileSync(this.path, JSON.stringify(content), { encoding: "utf-8" });
    }

    /**
     * Merges the input object with the next object being appended where appropriate to the existing object.
     */
    deepMerge(newContent: T): void {
        if (!fs.existsSync(this.path)) {
            throw new Error(`The ${this.path} file does not exist.`);
        }
        let existingContent = JSON.parse(
            fs.readFileSync(this.path, { encoding: "utf-8" }),
        );
        const newObj = deepMerge(existingContent, newContent);
        fs.writeFileSync(this.path, JSON.stringify(newObj), { encoding: "utf-8" });
    }
}
