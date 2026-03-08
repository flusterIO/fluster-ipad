export abstract class FileHelper<FileReadReturn> {
    path: string;
    constructor(filePath: string) {
        this.path = filePath;
    }
    abstract read(): FileReadReturn;

    abstract write(content: FileReadReturn): void;
}
