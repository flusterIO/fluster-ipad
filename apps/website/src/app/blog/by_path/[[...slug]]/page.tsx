import React, { type ReactNode } from "react";
import { slugToFilePath } from "@conundrum/ts/pathUtils";
import data from "../../../../features/cdrm/cdrm.json";
import { notFound } from "next/navigation";
import "@conundrum/generated/conundrum.css";

interface ByPagePageProps {
    params: Promise<{ slug: string[] }>;
}

const ByPagePage = async (props: ByPagePageProps): Promise<ReactNode> => {
    const p = await props.params;
    const item = data.files.find(
        (f) => f.relative_path === slugToFilePath(p.slug),
    );
    if (!item) {
        return notFound();
    }
    return (
        <div className="w-full min-h-screen">
            <div dangerouslySetInnerHTML={{ __html: item.html }} />
        </div>
    );
};

ByPagePage.displayName = "ByPagePage";

export default ByPagePage;
