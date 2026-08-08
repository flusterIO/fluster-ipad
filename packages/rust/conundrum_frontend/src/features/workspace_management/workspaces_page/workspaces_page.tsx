import { PageContainer } from "@/components/general/page_container";
import React, { useState, type ReactNode } from "react";
import { WorkspaceListItem } from "./workspace_item";
import { PlusSquareIcon } from "lucide-react";
import { AddWorkspaceDialog } from "./add_workspace_page/add_workspace_dialog";
import { rspc } from "@/app/rspc_client";
import { Button } from "@/components/shad/button";
import { useEventListener } from "@/state/hooks/use_event_listener";

declare global {
    // eslint-disable-next-line @typescript-eslint/consistent-type-definitions
    interface WindowEventMap {
        "workspace-add": CustomEvent<undefined>;
    }
}

export const WorkspacesPage = (): ReactNode => {
    const {
        data: workspaces,
        refetch,
        error,
    } = rspc.useQuery(
        [
            "crud.workspace.get_by_predicate",
            {
                predicate: null,
                pagination: {
                    page: 1,
                    per_page: 10,
                },
            },
        ],
        {
            refetchOnWindowFocus: true,
            refetchOnMount: true,
        },
    );
    const [showWorkspaceDialog, setShowWorkspaceDialog] = useState(false);
    const hasWorkspaces = Boolean(workspaces?.length);
    useEventListener("workspace-add", (e) => {
        refetch();
    });
    return (
        <PageContainer
            title="Workspace Management"
            subtitle={
                <>
                    Each <span className="italic">workspace</span> is a directory that
                    contains your notes and all resources that your notes depend on.
                </>
            }
            center={!hasWorkspaces}
            toolbarClasses="justify-end"
            toolbar={
                hasWorkspaces ? (
                    <>
                        <Button
                            onClick={() => {
                                setShowWorkspaceDialog(true);
                            }}
                            size={"icon"}
                            variant={"secondary"}
                        >
                            <PlusSquareIcon />
                        </Button>
                    </>
                ) : undefined
            }
        >
            {hasWorkspaces ? (
                <div className="w-full flex flex-col justify-start items-center gap-y-4 py-4">
                    {workspaces.map((item) => {
                        return <WorkspaceListItem key={item.root} workspace={item} />;
                    })}
                </div>
            ) : (
                <div className="w-full h-fit flex flex-col justify-center items-center">
                    <div className="mb-3">No workspaces to show</div>
                    <a
                        role="button"
                        className="bg-secondary rounded"
                        onClick={() => {
                            setShowWorkspaceDialog(true);
                        }}
                    >
                        <PlusSquareIcon className="w-10 h-10 rounded-xl text-secondary-foreground/60! hover:text-secondary-foreground! transition-colors duration-300 cursor-pointer" />
                    </a>
                </div>
            )}
            <AddWorkspaceDialog
                open={showWorkspaceDialog}
                close={() => {
                    setShowWorkspaceDialog(false);
                }}
            />
        </PageContainer>
    );
};

WorkspacesPage.displayName = "WorkspacesPage";
