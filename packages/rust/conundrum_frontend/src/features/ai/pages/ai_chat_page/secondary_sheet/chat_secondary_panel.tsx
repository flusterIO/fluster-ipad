import { SecondaryPanelKey } from "#/navigation/secondary_panel/secondary_panel_key";
import {
    Sheet,
    SheetContent,
    SheetContentNoPortal,
} from "@/components/shad/sheet";
import { type AppState } from "@/state/initial_state";
import React, { type ReactNode } from "react";
import { useSelector } from "react-redux";
import { AgentSelectionPanel } from "./sheets/agent_selection/agent_selection_panel";
import { ChatSelectionSheet } from "./sheets/chat_selection/chat_selection";
import { AnimatePresence } from "framer-motion";
import { ToolCallHistorySheet } from "./sheets/tool_call_history/tool_call_history_panel";
import { SheetTitle } from "./sheets/sheet_title";
import { motion } from "framer-motion";
import {
    useChatPageContext,
    useChatPageDispatch,
} from "../chat_page_context/chat_page_context";
import { cn } from "@/utils/shad_utils";

export const ChatSideSheet = (): ReactNode => {
    const { sheetOpen: open } = useChatPageContext();
    const dispatch = useChatPageDispatch();
    const close = () => {
        dispatch({
            type: "set-sheet-open",
            payload: false,
        });
    };
    const sheet = useSelector((state: AppState) => {
        return state.navigation.side_panel.active_panel;
    });
    console.log("open: ", open);
    return (
        <motion.div
            className="fixed w-screen h-screen top-0 right-0 bottom-0 left-0 bg-background/30"
            animate={open ? "open" : "closed"}
            initial={"closed"}
            variants={{
                closed: {
                    scale: 0,
                    opacity: 0,
                },
                open: {
                    scale: 1,
                    opacity: 1,
                },
            }}
            onClick={close}
        >
            <div className="w-full h-fit flex flex-col min-h-full justify-center">
                <motion.div
                    className={
                        "h-screen w-[min(350px,90vw)] border-l absolute top-0 right-0 bottom-0 bg-fd-card origin-right"
                    }
                    animate={open ? "open" : "closed"}
                    initial={"closed"}
                    variants={{
                        closed: {
                            x: "100%",
                            scale: 0,
                            opacity: 0,
                        },
                        open: {
                            x: 0,
                            scale: 1,
                            opacity: 1,
                        },
                    }}
                    transition={{
                        bounce: 0,
                    }}
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
                </motion.div>
            </div>
        </motion.div>
    );
};

ChatSideSheet.displayName = "ChatSelectionSheet";
