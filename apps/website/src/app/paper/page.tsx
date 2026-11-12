import { PdfComponentWrapped } from "#/features/docs/pdf_component/pdf_component_wrapped";
import React, { type ReactNode } from "react";

const PaperPage = (): ReactNode => {
    return (
        <div className="w-full h-fit min-h-screen flex flex-col justify-center items-center">
            <div className="mx-auto max-w-240 flex flex-col justify-start items-center my-8">
                <div className="w-fit flex flex-col justify-center items-center text-center">
                    <div className="text-lg font-semibold">Warning</div>
                    <div>
                        This is a working draft that has not yet been peer reviewed.
                    </div>
                </div>
                <PdfComponentWrapped file="/assets/pdf/paper_preprint.pdf" />
            </div>
        </div>
    );
};

PaperPage.displayName = "PaperPage";

export default PaperPage;
