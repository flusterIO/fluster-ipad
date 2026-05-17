import React, { type ReactNode } from "react";
import { type BlogSearchParams, BlogTemplate } from "@conundrum/ts/ui/blog";
import cdrmOutput from "../../../features/cdrm/cdrm.json";

interface BlogHomePageProps {
    searchParams: Promise<BlogSearchParams>;
}

const BlogHomePage = async ({
    searchParams,
}: BlogHomePageProps): Promise<ReactNode> => {
    const sp = await searchParams;
    return (
        <div className="w-full h-fit min-h-screen">
            <BlogTemplate sidebarItems={[]} searchParams={sp} data={cdrmOutput} />
        </div>
    );
};

BlogHomePage.displayName = "BlogHomePage";

export default BlogHomePage;
