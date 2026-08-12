import { GenericRemoteDataProvider } from "#/database/state/generic_data_loading_context/generic_data_loading_context";
import { logMaybeObject } from "#/error_handling/utils/log_maybe_object";
import { showErrorNotification } from "#/notifications/state/actions/show_notification";
import { client } from "@/app/rspc_client";
import React, { type ReactNode } from "react";
import { useSearchParams } from "react-router";

interface WorkspaceLoaderProps {
    children: ReactNode;
}

export const WorkspaceLoader = ({
    children,
}: WorkspaceLoaderProps): ReactNode => {
    const [sp] = useSearchParams();
    const root = sp.get("fs_path");
    return (
        <GenericRemoteDataProvider
            initialValues={{
                loading: true,
                data: null,
            }}
            loader={async () => {
                if (!root) {
                    showErrorNotification({
                        title: "Invalid URL",
                        body: "No file path to the workspace was provided",
                        timeout: 5000,
                    });
                    return null;
                }
                try {
                    const res = await client.query([
                        "crud.user_workspace.get_by_predicate",
                        {
                            predicate: `root="${root}"`,
                            pagination: {
                                page: 1,
                                per_page: 1,
                            },
                        },
                    ]);
                    return { workspace: res.length >= 1 ? res[0] : null };
                } catch (err: unknown) {
                    logMaybeObject("Error: ", err);
                }
            }}
        >
            {children}
        </GenericRemoteDataProvider>
    );
};
