import { DisabledLabeledText } from "#/settings/inputs/string_inputs/disabled_labeled_text";
import { Procedures } from "@/codegen/bindings_t";
import React, { type ReactNode } from "react";

interface WorkspaceListItemProps {
    workspace: Procedures["user_workspace_crud"]["get_by_predicate"]["output"][number];
}

export const WorkspaceListItem = ({
    workspace,
}: WorkspaceListItemProps): ReactNode => {
    return (
        <div className="@container/workspaceItem w-full flex flex-col justify-start items-start p-4 border rounded-xl">
            <div className="w-full grid grid-cols-1 @[640px]/workspaceItem:grid-cols-2">
                <DisabledLabeledText
                    label="Label"
                    content={
                        !workspace.label || workspace.label.trim() === ""
                            ? "--"
                            : workspace.label
                    }
                    classes={{
                        content: "overflow-x-auto overflow-y-hidden text-sm",
                    }}
                />
                <DisabledLabeledText
                    label="Path"
                    content={workspace.root}
                    classes={{
                        content: "overflow-x-auto overflow-y-hidden font-mono text-sm",
                    }}
                />
            </div>
        </div>
    );
};

WorkspaceListItem.displayName = "WorkspaceListItem";
