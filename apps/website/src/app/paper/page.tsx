import { PdfComponentWrapped } from "#/features/docs/pdf_component/pdf_component_wrapped";
import React, { type ReactNode } from "react";
import Link from "next/link";
import { JsonLd } from "#/core/seo/json_ld";
import { buildMetadata, paper } from "#/core/seo/site";
import { breadcrumbJsonLd, paperJsonLd } from "#/core/seo/structured_data";

export const metadata = buildMetadata({
    title: `${paper.shortTitle}: Relativity without a Time Dimension`,
    description: paper.description,
    path: paper.path,
    keywords: paper.keywords,
    type: "article",
});

const PaperPage = (): ReactNode => {
    return (
        <div className="w-full h-fit min-h-screen flex flex-col justify-center items-center">
            <JsonLd
                data={[
                    paperJsonLd,
                    breadcrumbJsonLd([
                        { name: "Fluster", path: "/" },
                        { name: "Paper", path: paper.path },
                    ]),
                ]}
            />
            <div className="mx-auto max-w-240 px-4 flex flex-col justify-start items-center my-8">
                <header className="w-full flex flex-col gap-y-3 text-center items-center">
                    <h1 className="text-3xl md:text-4xl font-bold">{paper.title}</h1>
                    <p className="text-muted-foreground">
                        Andrew Mueller &middot; CC BY 4.0 &middot; DOI{" "}
                        <a
                            className="text-primary underline"
                            href={`https://doi.org/${paper.doi}`}
                            rel="noopener"
                        >
                            {paper.doi}
                        </a>
                    </p>
                    <p className="max-w-3xl text-left">{paper.description}</p>
                    <nav className="flex flex-wrap justify-center gap-4 text-sm">
                        <a
                            className="text-primary underline"
                            href={paper.zenodo}
                            rel="noopener"
                        >
                            Published record on Zenodo
                        </a>
                        <a
                            className="text-primary underline"
                            href="/assets/pdf/paper_preprint.pdf"
                        >
                            Download PDF
                        </a>
                        <Link
                            className="text-primary underline"
                            href="/blog/by_path/on_the_gravitational_nature_of_time"
                        >
                            Visual walkthrough
                        </Link>
                        <Link
                            className="text-primary underline"
                            href="/blog/by_path/blog_posts/physics/lorentz_slope_visualizer"
                        >
                            Interactive companion app
                        </Link>
                    </nav>
                </header>
                <div className="w-fit flex flex-col justify-center items-center text-center mt-6">
                    <div className="text-lg font-semibold">Warning</div>
                    <div>
                        This is a working draft that has not yet been peer reviewed.
                    </div>
                </div>
                <PdfComponentWrapped file="/assets/pdf/paper_preprint.pdf" />
            </div>
        </div>
    );
};

PaperPage.displayName = "PaperPage";

export default PaperPage;
