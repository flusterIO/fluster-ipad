import React, { useMemo, type ReactNode } from "react";
import { type AnyBuilderOutput } from "../../types/general";

interface BlogRenderedPageProps {
    data: AnyBuilderOutput;
    slug: AnyBuilderOutput;
}

export const BlogRenderedPage = ({
    slug,
    data,
}: BlogRenderedPageProps): ReactNode => {
    const m = useMemo((): string | null => {
        data.files.forEach((n) => {
            console.log("n.relativePath: ", n.relative_path);
        });
        return null;
    }, [slug, data]);

    if (!m) {
        return null;
    }
    return <div dangerouslySetInnerHTML={{ __html: m }} />;
};

BlogRenderedPage.displayName = "BlogRenderedPage";
