import React, { type ReactNode } from "react";
import { rspc } from "@/app/rspc_client";
import { ViewCdrmPage } from "./view_cdrm_page";

interface ViewWorkspaceByPathProps {
    ws: string;
    rel: string;
}

export const ViewWorkspaceByPath = ({
    path,
}: ViewWorkspaceByPathProps): ReactNode => {
    /* const { data } = rspc.useMutation(["fs"]); */
    return <ViewCdrmPage content={""} />;
};

ViewWorkspaceByPath.displayName = "ViewWorkspaceByPath";
