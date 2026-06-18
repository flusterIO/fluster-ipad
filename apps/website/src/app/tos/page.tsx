import React, { type ReactNode } from "react";
import { files } from "../../features/cdrm/cdrm.json";
import { buttonVariants } from "../../core/shad/ui/button";
import Link from "next/link";

const TermsOfServicePage = (): ReactNode => {
    const tos = files.find(
        (f) => f.relative_path === "legal/terms_of_service.cdrm",
    );
    return (
        <div className="w-full h-fit min-h-screen flex flex-col justify-center items-center">
            <div
                className="@container/mdx @container/cdrm max-w-[1080px] mx-auto py-8 h-fit w-full px-6"
                dangerouslySetInnerHTML={{ __html: tos?.html ?? "" }}
            />
            <div className="w-full flex flex-row justify-end items-center max-w-[1080px] mx-auto">
                <Link href="/" className={buttonVariants()}>
                    Back to Fluster
                </Link>
            </div>
        </div>
    );
};

TermsOfServicePage.displayName = "TermsOfServicePage";

export default TermsOfServicePage;
