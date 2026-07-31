import consola from "consola";

export const getRequiredEnvVariable = (envVar: string): string | undefined => {
    const envVarValue = process.env[envVar];
    if (envVarValue) {
        return envVarValue;
    } else {
        consola.error(
            `Conundrum requires the \`${envVar}\` environment variable to properly function. Please review the documentation for more information.`,
        );
    }
};
