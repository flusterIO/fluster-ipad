import React, { type ReactNode } from "react";
import { files } from "../../features/cdrm/cdrm.json";
import { ClientConundrumPage } from "../blog/by_path/[[...slug]]/client_cdrm";
import { type BlogFileSummary } from "../../../../../packages/rust/conundrum_ts/dist/src/code_gen";
import Footer from "#/features/footer";

const SoulOfConundrumPage = (): ReactNode => {
    const page = files.find((f) => {
        return f.relative_path === "legal/soul.md";
    });
    return (
        <div className="w-full h-fit min-h-screen">
            <ClientConundrumPage item={page as BlogFileSummary} />
            <Footer />
        </div>
    );
};

SoulOfConundrumPage.displayName = "SoulOfConundrumPage";

export default SoulOfConundrumPage;
