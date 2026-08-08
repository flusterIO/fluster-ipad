import { DisabledLabeledText } from "#/settings/inputs/string_inputs/disabled_labeled_text";
import type { Procedures } from "@/codegen/bindings_t";
import React, { type ReactNode } from "react";
import { AppPaths } from "#/navigation/app_paths";
import { Link, useNavigate } from "react-router";
import { buttonVariants } from "@/components/shad/button";
import { cn } from "@/utils/shad_utils";
import { logMaybeObject } from "#/error_handling/utils/log_maybe_object";

interface WorkspaceListItemProps {
    workspace: Procedures["user_workspace_crud"]["get_by_predicate"]["output"][number];
}

export const WorkspaceListItem = ({
    workspace,
}: WorkspaceListItemProps): ReactNode => {
    const workspaceSp = new URLSearchParams();
    workspaceSp.set("fs_path", workspace.root);
    const navigate = useNavigate();
    return (
        <div
            key={workspace.root}
            className="@container/workspaceItem w-full flex flex-col justify-start items-start p-4 border rounded-xl bg-fd-card text-fd-card-foreground! cursor-pointer"
            onClick={async () => {
                navigate({
                    pathname: AppPaths.singleWorkspaceView,
                    search: workspaceSp.toString(),
                }).catch((err: unknown) => {
                    logMaybeObject("Error: ", err);
                });
            }}
        >
            <div className="w-full grid grid-cols-1 @[640px]/workspaceItem:grid-cols-2">
                <DisabledLabeledText
                    label="Label"
                    content={
                        !workspace.label || workspace.label.trim() === ""
                            ? "--"
                            : workspace.label
                    }
                    classes={{
                        content: "overflow-x-auto overflow-y-hidden text-sm bg-fd-card",
                    }}
                />
                <DisabledLabeledText
                    label="Path"
                    content={workspace.root}
                    classes={{
                        content:
                            "overflow-x-auto overflow-y-hidden font-mono text-sm bg-fd-card",
                    }}
                />
            </div>
            <div className="w-full flex flex-row justify-end items-center mt-3">
                <Link
                    to={{
                        pathname: AppPaths.singleWorkspaceManagement,
                        search: workspaceSp.toString(),
                    }}
                    onClick={(e) => {
                        e.stopPropagation();
                    }}
                    className={cn(
                        buttonVariants({ variant: "outline" }),
                        "bg-transparent",
                    )}
                >
                    Edit
                </Link>
            </div>
        </div>
    );
};

WorkspaceListItem.displayName = "WorkspaceListItem";
