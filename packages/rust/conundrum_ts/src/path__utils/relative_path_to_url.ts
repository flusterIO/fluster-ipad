export const relativePathToBlogUrl = (relativePath: string): string => {
    const li = relativePath.lastIndexOf(".");
    return `/blog/by_path/${relativePath.slice(0, li > 0 ? li : relativePath.length)}`;
};
