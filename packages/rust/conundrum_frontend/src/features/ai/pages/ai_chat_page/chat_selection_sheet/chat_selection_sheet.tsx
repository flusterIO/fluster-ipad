import { SecondaryPanelKey } from "#/navigation/secondary_panel/secondary_panel_key";
import { Sheet, SheetContent } from "@/components/shad/sheet";
import { type AppState } from "@/state/initial_state";
import React, { type ReactNode } from "react";
import { useSelector } from "react-redux";
import { AgentSelectionPanel } from "./sheets/agent_selection/agent_selection_panel";

export const ChatSideSheet = ({
    open,
    close,
}: {
    open: boolean;
    close: () => void;
}): ReactNode => {
    const sheet = useSelector((state: AppState) => {
        return state.navigation.side_panel.active_panel;
    });
    return (
        <Sheet
            open={open}
            onOpenChange={(val) => {
                if (!val) {
                    close();
                }
            }}
        >
            <SheetContent
                side="right"
                className="flex flex-col justify-start items-center"
            >
                {sheet === SecondaryPanelKey.AgentSelect ? (
                <AgentSelectionPanel />
                ) : (
                < />
                )}
            </SheetContent>
        </Sheet>
    );
};

ChatSideSheet.displayName = "ChatSelectionSheet";
