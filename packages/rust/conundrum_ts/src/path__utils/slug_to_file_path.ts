export const slugToFilePath = (
    slug: string[],
    seperator = "/",
    extension: `.${string}` = ".cdrm",
): string => {
    return `${slug.join(seperator)}${extension}`;
};
