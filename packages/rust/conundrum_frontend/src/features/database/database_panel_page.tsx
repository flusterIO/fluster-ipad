import React, { type ReactNode } from "react";
import { DatabaseTable } from "./database_table/database_table";
import { PageContainer } from "@/components/general/page_container";

export const DatabasePanelPage = (): ReactNode => {
    return (
        <PageContainer title="Database">
            <DatabaseTable />
        </PageContainer>
    );
};

DatabasePanelPage.displayName = "DatabasePanelPage";
