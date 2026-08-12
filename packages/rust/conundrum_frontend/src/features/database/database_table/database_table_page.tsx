import React, { type ReactNode } from "react";
import { DatabaseTable } from "./database_table";
import { DatabaseTableProvider } from "./database_table_context/database_table_context";


export const DatabaseTablePage = (): ReactNode => {
    return (
        <div className="flex flex-col justify-center items-center w-full h-full min-h-[calc(100vh-4rem)] px-4 md:px-6 py-4">
            <div className="w-full max-w-360 mx-auto">
                <DatabaseTableProvider>
                    <DatabaseTable />
                </DatabaseTableProvider>
            </div>
        </div>
    );
};

DatabaseTablePage.displayName = "DatabaseTablePage";
