import { baseOptions } from "#/core/mdx/base_options";
import { RootToggle } from "fumadocs-ui/components/layout/root-toggle";
import { DocsLayout } from "fumadocs-ui/layouts/docs";
import { UserIcon } from "lucide-react";
import type { ReactNode } from "react";
import "../../../core/styles/docs.scss";
import { source } from "#/core/mdx/sources/fumadocs_mdx/docs";

export default function Layout({ children }: { children: ReactNode }) {
    return (
        <DocsLayout
            tree={source.pageTree}
            {...baseOptions}
            containerProps={{
                className:
                    "[&_#nd-sidebar>div[data-fdid]]:w-full [&_a[data-active]]:before:content-none",
            }}
            githubUrl="https://github.com/flusterIO"
            sidebar={{
                banner: (
                    <RootToggle
                        className="py-0 relative [&>div]:flex [&>div]:flex-col [&>div]:items-center [&>div]:justify-center"
                        /* popover="auto" */
                        options={[
                            {
                                title: "User",
                                description: "For everyone",
                                url: "/docs/user",
                                props: {
                                    /* className: */
                                    /*     "relative flex flex-col justify-center items-center", */
                                    /* className: */
                                    /*     "bg-popover hover:bg-muted/80 transition-colors duration-150", */
                                },
                                icon: <UserIcon className="w-5 h-5" />,
                            },
                        ]}
                    />
                ),
            }}
        >
            {children}
        </DocsLayout>
    );
}
