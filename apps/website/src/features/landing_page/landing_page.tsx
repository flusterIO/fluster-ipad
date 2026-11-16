import React, { type ReactNode } from "react";
import { LandingPageClientContent } from "./landing_page_client_content";
/* import { nestedContentSource } from "#/core/mdx/sources/fumadocs_mdx/nested_mdx"; */
/* import { notFound } from "next/navigation"; */

const LandingPage = (): ReactNode => {
    return (
        <div className="w-full min-h-screen pb-8 flex flex-col justify-start items-center overflow-x-hidden">
            <LandingPageClientContent />
        </div>
    );
};

LandingPage.displayName = "LandingPage";

export default LandingPage;
