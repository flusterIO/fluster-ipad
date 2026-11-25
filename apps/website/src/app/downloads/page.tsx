import PageNotYetBuilt from "#/features/coming_soon/page_not_yet_built";
import React, { type ReactNode } from "react";
import { buildMetadata } from "#/core/seo/site";

export const metadata = buildMetadata({
    title: "Downloads",
    description: "Download Fluster for iPad and Mac.",
    path: "/downloads",
    noIndex: true,
});


const DownloadsPage = (): ReactNode => {
    return (
        <div className="w-screen min-h-screen flex flex-col justify-center items-center">
            <PageNotYetBuilt />
        </div>
    );
};

DownloadsPage.displayName = "DownloadsPage";

export default DownloadsPage;
