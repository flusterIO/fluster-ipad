import { SecondaryPanelKey } from "#/navigation/secondary_panel/secondary_panel_key";
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
import { ChatRelatedLogsPanel } from "./sheets/chat_related_logs/chat_related_logs";
import { useSearchParams } from "react-router";
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
    const [sp] = useSearchParams();
    const conversationId = sp.get("convo");
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
                    className={cn(
                        "h-screen w-[min(350px,90vw)] border-l absolute top-0 right-0 bottom-0 bg-fd-card origin-right flex flex-col",
                    )}
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
                    onClick={(e) => {
                        e.stopPropagation();
                    }}
                >
                    <SheetTitle />
                    <AnimatePresence key={sheet}>
                        {conversationId ? (
                            sheet === SecondaryPanelKey.AgentSelect ? (
                                <AgentSelectionPanel key="agent-selection" />
                            ) : sheet === SecondaryPanelKey.ToolExecHistory ? (
                                <ToolCallHistorySheet
                                    convo={conversationId}
                                    key="tool-call-history"
                                />
                            ) : sheet === SecondaryPanelKey.Logs ? (
                                <ChatRelatedLogsPanel key="chat-logs" />
                            ) : (
                                <ChatSelectionSheet key="chat-selection" />
                            )
                        ) : (
                            <div className="grow">
                                <h3>No Conversation</h3>
                                <p className="text-foreground/80 font-sm">
                                    You must be in an active conversation to use this panel
                                </p>
                            </div>
                        )}
                    </AnimatePresence>
                </motion.div>
            </div>
        </motion.div>
    );
};

ChatSideSheet.displayName = "ChatSelectionSheet";
