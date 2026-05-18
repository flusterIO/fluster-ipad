import React, { type ReactNode } from "react";
import { type BlogSearchParams } from "@conundrum/ts/ui/blog";
import { BlogTemplateClientWrapper } from "./client_cdrm";
import { SidebarBanner } from "#/features/blog/sidebar/banner";

interface BlogHomePageProps {
    searchParams: Promise<BlogSearchParams>;
    params: Promise<{ slug: string[] }>;
}

enum SidebarCategories {
    Documentation = "Documentation",
    MyWork = "My Work",
}

const BlogHomePage = async ({
    searchParams,
    params,
}: BlogHomePageProps): Promise<ReactNode> => {
    const sp = await searchParams;
    const p = await params;
    console.log("p: ", p);
    return (
        <div className="w-full h-fit min-h-screen">
            <BlogTemplateClientWrapper
                sidebarItems={[
                    {
                        category: SidebarCategories.Documentation,
                        href: "/docs/documentation",
                        label: "Documentation",
                        id: "documentation",
                    },
                    {
                        category: SidebarCategories.MyWork,
                        href: "/docs?id=on_the_gravitational_nature_of_time",
                        label: "On the gravitational nature of time",
                        id: "my-work",
                    },
                ]}
                searchParams={sp}
                slug={p.slug}
                banner={<SidebarBanner />}
            />
        </div>
    );
};

BlogHomePage.displayName = "BlogHomePage";

export default BlogHomePage;
