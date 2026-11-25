import type { Metadata } from "next";

/**
 * Single source of truth for SEO. The site currently serves both Fluster (the
 * app) and Conundrum (the language), plus the associated paper, so every
 * default here is written to cover all three.
 */
export const siteUrl = "https://flusterapp.com";

export const seo = {
    siteName: "Fluster",
    title: "Fluster | Academic Note Taking for iPad & Mac, powered by Conundrum",
    titleTemplate: "%s | Fluster",
    description:
        "Fluster is a free, open source academic note taking app for iPad and Mac. Hand-drawn notes with PaperKit, citations, math, tags and search, powered by Conundrum, an MDX-based markup language.",
    keywords: [
        "Fluster",
        "Conundrum",
        "academic note taking",
        "iPad note taking app",
        "Apple Pencil notes",
        "PaperKit",
        "STEM notes",
        "LaTeX math notes",
        "BibLaTeX citations",
        "Zotero alternative",
        "Obsidian alternative",
        "MDX notes",
        "open source note taking",
        "Conundrum language",
        "research notes",
    ],
    author: "Andrew Mueller",
    twitterHandle: "@fluster_app",
} as const;

export const paper = {
    path: "/paper",
    title: "Absolute Time in Higher-Dimensional Geometry: Classically Simultaneous Relativity via an Unwrapped Kaluza-Klein Dimension",
    shortTitle: "Absolute Time in Higher-Dimensional Geometry",
    doi: "10.5281/zenodo.24504695",
    zenodo: "https://zenodo.org/records/24504695",
    description:
        "Special relativity written with a fourth spatial axis and no time dimension. Gravity follows from one co-expansion postulate, agrees with general relativity at first post-Newtonian order and departs from it at second order.",
    keywords: [
        "absolute time",
        "Kaluza-Klein",
        "special relativity",
        "general relativity alternative",
        "Lorentz factor",
        "equivalence principle",
        "Painlevé-Gullstrand coordinates",
        "preferred frame",
        "second-order light deflection",
    ],
} as const;

export const absoluteUrl = (path: string): string =>
    new URL(path, siteUrl).toString();

/**
 * Builds page metadata with a canonical URL and matching OpenGraph/Twitter
 * fields. Social images fall back to the root `opengraph-image.png`.
 */
export const buildMetadata = ({
    title,
    description,
    path,
    keywords,
    type = "website",
    noIndex = false,
}: {
    title: string;
    description: string;
    path: string;
    keywords?: readonly string[];
    type?: "website" | "article";
    noIndex?: boolean;
}): Metadata => ({
    title,
    description,
    keywords: keywords ? [...keywords] : undefined,
    alternates: { canonical: path },
    openGraph: {
        title,
        description,
        url: path,
        type,
        siteName: seo.siteName,
        locale: "en_US",
    },
    twitter: { card: "summary_large_image", title, description },
    robots: noIndex ? { index: false, follow: false } : undefined,
});
