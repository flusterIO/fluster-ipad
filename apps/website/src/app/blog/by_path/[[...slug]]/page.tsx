import React, { type ReactNode } from "react";
import { slugToFilePath } from "@conundrum/ts/pathUtils";
import data from "../../../../features/cdrm/cdrm.json";
import { notFound } from "next/navigation";
import { ClientConundrumPage } from "./client_cdrm";
import { type AnyBuilderOutput } from "../../../../../../../packages/rust/conundrum_ts/dist/src/types/general";
import { ConundrumLoader } from "#/features/cdrm/cdrm_loader";
import type { Metadata } from "next";
import { JsonLd } from "#/core/seo/json_ld";
import { absoluteUrl, buildMetadata, paper, seo } from "#/core/seo/site";
import { blogEntries, findBlogEntry } from "#/core/seo/blog_entries";
import { breadcrumbJsonLd } from "#/core/seo/structured_data";

interface ByPagePageProps {
    params: Promise<{ slug?: string[] }>;
}

export const dynamicParams = false;

export const generateStaticParams = (): { slug: string[] }[] =>
    blogEntries.map((e) => ({ slug: e.slug }));

export const generateMetadata = async (
    props: ByPagePageProps,
): Promise<Metadata> => {
    const entry = findBlogEntry((await props.params).slug);
    if (!entry) {
        return { robots: { index: false, follow: false } };
    }
    return buildMetadata({
        title: entry.title,
        description: entry.description ?? seo.description,
        path: entry.path,
        type: "article",
        keywords: entry.subject === "Physics" ? paper.keywords : seo.keywords,
    });
};

const ByPagePage = async (props: ByPagePageProps): Promise<ReactNode> => {
    const p = await props.params;
    const item = data.files.find(
        (f) => f.relative_path === slugToFilePath(p.slug ?? []),
    );
    if (!item) {
        return notFound();
    }
    const entry = findBlogEntry(p.slug);
    return (
        <div className="@container/mdx @container/cdrm mdx w-full overflow-x-hidden overflow-y-auto! max-h-screen min-h-screen px-6">
            {entry && (
                <JsonLd
                    data={[
                        {
                            "@context": "https://schema.org",
                            "@type": "Article",
                            headline: entry.title,
                            description: entry.description ?? undefined,
                            url: absoluteUrl(entry.path),
                            mainEntityOfPage: absoluteUrl(entry.path),
                            articleSection: entry.subject ?? undefined,
                            inLanguage: "en",
                            author: { "@type": "Person", name: seo.author },
                            publisher: { "@id": absoluteUrl("/#organization") },
                            ...(entry.subject === "Physics"
                                ? { citation: absoluteUrl(paper.path) }
                                : {}),
                        },
                        breadcrumbJsonLd([
                            { name: "Fluster", path: "/" },
                            { name: "Blog", path: "/blog" },
                            { name: entry.title, path: entry.path },
                        ]),
                    ]}
                />
            )}
            <ConundrumLoader />
            <ClientConundrumPage
                item={item as unknown as AnyBuilderOutput["files"][number]}
            />
        </div>
    );
};

ByPagePage.displayName = "ByPagePage";

export default ByPagePage;
