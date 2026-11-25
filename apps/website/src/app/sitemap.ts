import type { MetadataRoute } from "next";
import { absoluteUrl, paper } from "#/core/seo/site";
import { blogEntries } from "#/core/seo/blog_entries";

const staticRoutes: {
    path: string;
    changeFrequency: "weekly" | "monthly" | "yearly";
    priority: number;
}[] = [
    { path: "/", changeFrequency: "weekly", priority: 1 },
    { path: paper.path, changeFrequency: "monthly", priority: 0.9 },
    { path: "/blog", changeFrequency: "weekly", priority: 0.8 },
    { path: "/soul", changeFrequency: "yearly", priority: 0.4 },
    { path: "/license", changeFrequency: "yearly", priority: 0.2 },
    { path: "/privacy", changeFrequency: "yearly", priority: 0.2 },
    { path: "/tos", changeFrequency: "yearly", priority: 0.2 },
];

export default function sitemap(): MetadataRoute.Sitemap {
    const lastModified = new Date();
    return [
        ...staticRoutes.map((r) => ({
            url: absoluteUrl(r.path),
            lastModified,
            changeFrequency: r.changeFrequency,
            priority: r.priority,
        })),
        ...blogEntries.map((e) => ({
            url: absoluteUrl(e.path),
            lastModified,
            changeFrequency: "monthly" as const,
            // The paper walkthrough is the main funnel to the paper.
            priority:
                e.slug.join("/") === "on_the_gravitational_nature_of_time"
                    ? 0.9
                    : 0.7,
        })),
    ];
}
