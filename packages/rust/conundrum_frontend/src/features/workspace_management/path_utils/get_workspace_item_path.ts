import { AppPaths } from "#/navigation/app_paths";

/**
 * Returns the url to the page of an individual workspace.
 */
export const getWorkspaceManagementPath = (workspacePath: string): string => {
    const sp = new URLSearchParams();
    sp.set("fs_path", workspacePath);
    return `${AppPaths.singleWorkspaceManagement}?${sp.toString()}`;
};
