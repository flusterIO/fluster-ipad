import { useSearchParamsObject } from "#/search/state/hooks/use_search_params_object";
import { PageContainer } from "@/components/general/page_container";
import React, { type ReactNode } from "react";

export const ViewConundrumPage = (): ReactNode => {
    const sp = useSearchParamsObject();
    return <PageContainer>Conundrum Content</PageContainer>;
};

ViewConundrumPage.displayName = "ViewConundrumPage";
