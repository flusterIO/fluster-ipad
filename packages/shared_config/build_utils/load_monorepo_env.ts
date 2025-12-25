import fs from 'fs';
import path from 'path';

export const getMonorepoRootDir = (): string => {
    let x = process.env.FLUSTER_IOS_ROOT;
    if (!x) {
        console.error("Cannot continue without the FLUSTER_IOS_ROOT venv variable set to the root of your workspace directory.")
        process.exit()
    }
    return x
}


export const loadMonorepoEnv = (): void => {
    let root = getMonorepoRootDir()
    let envPath = path.join(root, ".env.local");

    if (fs.existsSync(envPath)) {
        process.loadEnvFile(envPath)
    } else {
        console.warn("No monorepo-local env file found.")
    }
}
