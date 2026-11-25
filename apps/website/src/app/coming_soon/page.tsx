import PageNotYetBuilt from "#/features/coming_soon/page_not_yet_built";
import React, { type ReactNode } from "react";
import { buildMetadata } from "#/core/seo/site";

export const metadata = buildMetadata({
    title: "Coming soon",
    description: "This page is not built yet.",
    path: "/coming_soon",
    noIndex: true,
});


const ComingSoonPage = (): ReactNode => {
    return (
        <div className="w-screen min-h-screen flex flex-col justify-center items-center">
            <PageNotYetBuilt />
        </div>
    );
};

ComingSoonPage.displayName = "ComingSoonPage";

export default ComingSoonPage;
