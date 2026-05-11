"use client";
/* import { getMDXComponents } from "#/core/mdx/mdx_component_map"; */
/* import { source } from "#/core/mdx/sources/fumadocs_mdx/docs"; */
/* import { */
/*     DocsBody, */
/*     DocsDescription, */
/*     DocsPage, */
/*     DocsTitle, */
/* } from "fumadocs-ui/page"; */
/* import { notFound } from "next/navigation"; */
/* import { Admonition, Ul, Hl, AutoInsertedHeading, Hr } from "./client_mdx"; */
/* import { BlogHero } from "../../../features/blog/hero/blog_hero"; */
/* import { IPadCarousel } from "#/features/landing_page/sections/ipad_section/ipad_screenshot_carousel/carousel"; */
/* import { ConundrumInitializer } from "#/features/docs/conundrum_initializer"; */
/* import { onCopyCodeBlockClick } from "../../../../../../packages/rust/conundrum_ts/dist/src/component_glue/code/on_copy_code"; */

import { createPortal } from "react-dom";

/* export default async function Page(props: { */
/*     params: Promise<{ slug?: string[] }>; */
/* }) { */
/*     const params = await props.params; */
/*     const page = source.getPage(params.slug); */
/*     if (!page) { */
/*         notFound(); */
/*     } */
/*     const MDX = page.data.body; */
/*     return ( */
/*         <DocsPage toc={page.data.toc} full={page.data.full}> */
/*             <DocsTitle>{page.data.title}</DocsTitle> */
/*             <DocsDescription>{page.data.description}</DocsDescription> */
/*             <DocsBody className="@container/mdx prose prose-invert! w-full max-w-[1080px] px-8 prose-code:before:content-none prose-code:after:content-none prose-code:bg-[--shiki-light-bg] dark:prose-code:bg-[--shiki-dark-bg] [&_code_*]:text-[--shiki-light] dark:[&_code_*]:text-[--shiki-dark]"> */
/*                 <MDX */
/*                     components={getMDXComponents({ */
/*                         Admonition, */
/*                         Ul, */
/*                         Hl, */
/*                         AutoInsertedHeading, */
/*                         BlogHero, */
/*                         Hr, */
/*                         ImageCarousel: IPadCarousel, */
/*                     })} */
/*                 /> */
/*                 <ConundrumInitializer /> */
/*             </DocsBody> */
/*         </DocsPage> */
/*     ); */
/* } */

/* export async function generateStaticParams() { */
/*     return source.generateParams(); */
/* } */

/* export async function generateMetadata(props: { */
/*     params: Promise<{ slug?: string[] }>; */
/* }) { */
/*     const params = await props.params; */
/*     const page = source.getPage(params.slug); */
/*     if (!page) notFound(); */
/*     return { */
/*         title: page.data.title, */
/*         description: page.data.description, */
/*     }; */
/* } */

/// Start temporary replacement

import React, { type ReactNode } from "react";

const BodyPortal = ({ children }: { children: ReactNode }) => {
    if (!(document as typeof document | undefined)) {
        return null;
    }
    return createPortal(children, document.body);
};

const TemporaryDocsPage = (): ReactNode => {
    return (
        <BodyPortal>
            <div className="w-screen min-h-screen flex flex-col justify-center items-center bg-background/40 z-10">
                <div className="max-w-[min(90vw,540px)] fixed top-[1/2] left-[1/2] translate-x-[-1/2] translate-y-[-1/2] bg-fd-card text-fd-card-foreground border px-3 py-2">
                    <div className="text-xl md:text-2xl font-bold">
                        You're here at an odd time
                    </div>
                    <div className="mt-2">
                        Fluster is undergoing some major changes, and you're here right
                        while they're being implemented. Due to the major break through I
                        had with the model that I've been working on right when I'm about to
                        release this application, I decided to pull down the previous
                        documentation and all of it's flaws, while the new documentation
                        will be up in time for the new application's launch in the coming
                        weeks.
                    </div>
                    <div className="mt-3">Hang in there, it'll be worth it...</div>
                </div>
            </div>
        </BodyPortal>
    );
};

TemporaryDocsPage.displayName = "TemporaryDocsPage";

export default TemporaryDocsPage;
