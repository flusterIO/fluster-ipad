import { type PackageJson } from "../src/classes/package_json";
import path from "path";

const modifyPackageJsons = (packageJsons: PackageJson[]): PackageJson[] => {
    for (const p of packageJsons) {
        const root = p.projectRootDir();
        console.log(root.rootDir);
        console.log(`git restore HEAD ${path.join(root.rootDir, "tsconfig.json")}`);
        // const specialFileCntent = root.readSpecialFile(SpecialFile.eslint);

        // const relPath = path.relative(
        //     root.rootDir,
        //     path.join(process.env.FLUSTER_IOS_ROOT!, "tsconfig.json"),
        // );
        // console.log("relPath: ", relPath);
        // if (fs.existsSync(relPath)) {
        //     fs.rmSync(relPath);
        // }
    }
    return packageJsons;
};

export default modifyPackageJsons;
