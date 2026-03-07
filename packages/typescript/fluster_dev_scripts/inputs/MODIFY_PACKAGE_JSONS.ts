import { PackageJson } from "../src/classes/package_json";

const modifyPackageJsons = (packageJsons: PackageJson[]): PackageJson[] => {
    for (const p of packageJsons) {
        p.data.packageManager = "pnpm@9.7.1";
        p.data.version = "0.0.1";
    }
    return packageJsons;
};

export default modifyPackageJsons;
