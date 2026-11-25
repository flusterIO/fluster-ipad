import React, { type ReactNode } from "react";
import Link from "next/link";
import { paper } from "#/core/seo/site";

/**
 * Crawlable, text-first pointer to the paper and the Conundrum language, so the
 * home page passes link equity to both.
 */
export const PaperSection = (): ReactNode => {
    return (
        <section id="research" className="pt-20 px-4 w-full">
            <div className="mx-auto w-full max-w-[min(1080px,90vw)] flex flex-col gap-y-6">
                <div className="text-center">
                    <h2 className="text-3xl md:text-4xl font-bold text-gray-100 mb-4">
                        Built for research, and built from it
                    </h2>
                    <p className="text-lg text-gray-300 max-w-3xl mx-auto">
                        Fluster is written in{" "}
                        <Link
                            className="text-primary underline"
                            href="/blog/by_path/conundrum/syntax"
                        >
                            Conundrum
                        </Link>
                        , an open markup language for math-heavy notes. It also powers the
                        interactive companion to our paper, <em>{paper.title}</em>.
                    </p>
                </div>
                <div className="flex flex-wrap justify-center gap-4">
                    <Link className="text-primary underline" href={paper.path}>
                        Read the paper
                    </Link>
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
                    <a
                        className="text-primary underline"
                        href={`https://doi.org/${paper.doi}`}
                        rel="noopener"
                    >
                        DOI {paper.doi}
                    </a>
                </div>
            </div>
        </section>
    );
};

PaperSection.displayName = "PaperSection";
