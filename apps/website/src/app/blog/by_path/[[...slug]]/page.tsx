import React, { type ReactNode } from "react";
import { slugToFilePath } from "@conundrum/ts/pathUtils";
import data from "../../../../features/cdrm/cdrm.json";
import { notFound } from "next/navigation";
import { ClientConundrumPage } from "./client_cdrm";
import { type AnyBuilderOutput } from "../../../../../../../packages/rust/conundrum_ts/dist/src/types/general";
import { ConundrumLoader } from "#/features/cdrm/cdrm_loader";

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
        <div className="@container/mdx @container/cdrm mdx w-full overflow-x-hidden overflow-y-auto! max-h-screen min-h-screen px-6">
            <ConundrumLoader />
            <ClientConundrumPage
                item={item as unknown as AnyBuilderOutput["files"][number]}
            />
        </div>
    );
};

ByPagePage.displayName = "ByPagePage";

export default ByPagePage;
