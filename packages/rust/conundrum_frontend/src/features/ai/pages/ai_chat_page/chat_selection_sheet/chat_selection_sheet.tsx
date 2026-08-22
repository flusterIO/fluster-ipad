import { SecondaryPanelKey } from "#/navigation/secondary_panel/secondary_panel_key";
import { Sheet, SheetContent } from "@/components/shad/sheet";
import { type AppState } from "@/state/initial_state";
import React, { type ReactNode } from "react";
import { useSelector } from "react-redux";
import { AgentSelectionPanel } from "./sheets/agent_selection/agent_selection_panel";
import { ChatSelectionSheet } from "./sheets/chat_selection/chat_selection";
import { AnimatePresence } from "framer-motion";
import { ToolCallHistorySheet } from "./sheets/tool_call_history/tool_call_history_panel";
import { SheetTitle } from "./sheets/sheet_title";
import { useChatPageContext, useChatPageDispatch } from "../chat_page_context/chat_page_context";

export const ChatSideSheet = (): ReactNode => {
    const { sheetOpen: open } = useChatPageContext();
    const dispatch = useChatPageDispatch();
    const close = () => {
        dispatch({
            type: "set-sheet-open",
            payload: false
        })
    }
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
                <SheetTitle />
                <AnimatePresence key={sheet}>
                    {sheet === SecondaryPanelKey.AgentSelect ? (
                        <AgentSelectionPanel key="agent-selection" />
                    ) : sheet === SecondaryPanelKey.ToolExecHistory ? (
                        <ToolCallHistorySheet key="tool-call-history" />
                    ) : (
                        <ChatSelectionSheet key="chat-selection" />
                    )}
                </AnimatePresence>
            </SheetContent>
        </Sheet>
    );
};

ChatSideSheet.displayName = "ChatSelectionSheet";
