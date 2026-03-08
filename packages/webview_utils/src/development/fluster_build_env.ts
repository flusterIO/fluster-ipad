/**
 * The `FLUSTER_BUILD_ENV` variable made type-safe
 */
export type FlusterBuildEnvironment = "ipad" | "macos"


export interface FlusterBuildEnvironmentVariables {
    readonly FLUSTER_BUILD_ENV: FlusterBuildEnvironment;
    readonly FLUSTER_PROD_BUILD: "true" | "false" | undefined;
}
