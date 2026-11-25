import { staticContent } from "#/core/static_content";
import { absoluteUrl, paper, seo, siteUrl } from "./site";

const author = {
    "@type": "Person",
    name: seo.author,
    url: staticContent.links.social.github,
};

export const organizationJsonLd = {
    "@context": "https://schema.org",
    "@type": "Organization",
    "@id": `${siteUrl}/#organization`,
    name: "Fluster",
    url: siteUrl,
    logo: absoluteUrl("/assets/logo/fluster_banner_with_name.png"),
    sameAs: [
        staticContent.links.social.github,
        staticContent.links.social.twitter,
        staticContent.links.social.bluesky,
        staticContent.links.social.subreddit,
        staticContent.links.social.discord,
    ],
};

export const websiteJsonLd = {
    "@context": "https://schema.org",
    "@type": "WebSite",
    "@id": `${siteUrl}/#website`,
    url: siteUrl,
    name: "Fluster & Conundrum",
    description: seo.description,
    inLanguage: "en",
    publisher: { "@id": `${siteUrl}/#organization` },
};

export const softwareApplicationJsonLd = {
    "@context": "https://schema.org",
    "@type": "SoftwareApplication",
    "@id": `${siteUrl}/#app`,
    name: "Fluster",
    alternateName: "Fluster: Academic Note Taking",
    description: seo.description,
    url: siteUrl,
    applicationCategory: "EducationalApplication",
    applicationSubCategory: "Note taking",
    operatingSystem: "iPadOS, macOS",
    installUrl: staticContent.links.appstore.desktopHomePage,
    downloadUrl: staticContent.links.appstore.desktopHomePage,
    codeRepository: staticContent.links.social.github,
    license: absoluteUrl("/license"),
    isAccessibleForFree: true,
    offers: { "@type": "Offer", price: "0", priceCurrency: "USD" },
    author,
    publisher: { "@id": `${siteUrl}/#organization` },
    featureList: [
        "Hand-drawn notes with Apple PaperKit and Apple Pencil",
        "Conundrum, an MDX-based markup language for notes",
        "LaTeX math rendering",
        "BibLaTeX citations with CSL formatting",
        "Tags, topics, subjects and full-text search",
        "Local-first, free and open source",
    ],
    video: staticContent.links.videoDemo,
};

export const conundrumJsonLd = {
    "@context": "https://schema.org",
    "@type": "ComputerLanguage",
    "@id": `${siteUrl}/#conundrum`,
    name: "Conundrum",
    description:
        "Conundrum is a superset of MDX that transpiles academic notes to HTML, markdown, plain text or AI-ready output.",
    url: absoluteUrl("/blog/by_path/conundrum/syntax"),
    author,
};

export const paperJsonLd = {
    "@context": "https://schema.org",
    "@type": "ScholarlyArticle",
    "@id": `${siteUrl}${paper.path}#article`,
    headline: paper.title,
    name: paper.title,
    abstract: paper.description,
    url: absoluteUrl(paper.path),
    identifier: {
        "@type": "PropertyValue",
        propertyID: "DOI",
        value: paper.doi,
    },
    sameAs: [paper.zenodo, `https://doi.org/${paper.doi}`],
    keywords: paper.keywords.join(", "),
    author,
    license: "https://creativecommons.org/licenses/by/4.0/",
    inLanguage: "en",
    encoding: {
        "@type": "MediaObject",
        contentUrl: absoluteUrl("/assets/pdf/paper_preprint.pdf"),
        encodingFormat: "application/pdf",
    },
};

export const breadcrumbJsonLd = (
    items: { name: string; path: string }[],
): object => ({
    "@context": "https://schema.org",
    "@type": "BreadcrumbList",
    itemListElement: items.map((item, i) => ({
        "@type": "ListItem",
        position: i + 1,
        name: item.name,
        item: absoluteUrl(item.path),
    })),
});
