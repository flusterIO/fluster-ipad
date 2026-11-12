import { CdrmContent } from "#/cdrm/cdrm_content";
import React, { type ReactNode } from "react";

interface ViewCdrmPageProps {
  content: string;
}

export const ViewCdrmPage = ({ content }: ViewCdrmPageProps): ReactNode => {
  return (
    <div className="w-full max-w-[1080px] mx-auto px-4 sm:px-6 max-h-screen overflow-auto">
      <CdrmContent content={content} />
    </div>
  );
};

ViewCdrmPage.displayName = "ViewCdrmPage";
