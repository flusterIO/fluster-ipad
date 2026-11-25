import data from "#/features/cdrm/cdrm.json";

export interface BlogEntry {
    /** Path segments after `/blog/by_path/`. */
    slug: string[];
    path: string;
    title: string;
    description: string | null;
    subject: string | null;
}

/** Published, titled `.cdrm` notes outside the legal folder. */
export const blogEntries: BlogEntry[] = data.files.flatMap((f) => {
    const fm = f.front_matter as {
        title?: string | null;
        summary?: string | null;
        subject?: string | null;
    } | null;
    if (
        !f.relative_path.endsWith(".cdrm") ||
        f.relative_path.startsWith("legal/") ||
        !fm?.title
    ) {
        return [];
    }
    const slug = f.relative_path.replace(/\.cdrm$/, "").split("/");
    return [
        {
            slug,
            path: `/blog/by_path/${slug.join("/")}`,
            title: fm.title,
            description: fm.summary ?? null,
            subject: fm.subject ?? null,
        },
    ];
});

export const findBlogEntry = (
    slug: string[] | undefined,
): BlogEntry | undefined =>
    slug
        ? blogEntries.find((e) => e.slug.join("/") === slug.join("/"))
        : undefined;
