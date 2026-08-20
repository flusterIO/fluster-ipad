import React, { type ReactNode } from "react";
import { files } from "../../features/cdrm/cdrm.json";
import { ClientConundrumPage } from "../blog/by_path/[[...slug]]/client_cdrm";
import { type AnyBuilderOutput } from "../../../../../packages/rust/conundrum_ts/dist/src/types/general";
import Footer from "#/features/footer";

const LicensePage = (): ReactNode => {
    const item = files.find((f) => {
        console.log("f: ", f);
        return f.relative_path === "legal/wct_license.md";
    });
    return (
        <div className="w-full h-fit min-h-screen">
            <ClientConundrumPage
                item={item as unknown as AnyBuilderOutput["files"][number]}
            />
            <Footer />
        </div>
    );
};

LicensePage.displayName = "LicensePage";

export default LicensePage;
