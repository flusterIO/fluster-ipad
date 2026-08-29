import { SearchIcon } from "lucide-react";
import { motion } from "framer-motion";
import React, { type ReactNode } from "react";
import { CurrentlyStreamingMessage } from "./currently_streaming_message/currently_streaming_message";
import { Button } from "@/components/shad/button";
import { EmptyChat } from "./empty_chat/empty_chat";
import { ChatContent } from "./chat_content";
import { cn } from "@/utils/shad_utils";
import {
  ChatPageProvider,
  useChatPageContext,
  useChatPageDispatch,
} from "./chat_page_context/chat_page_context";
import { ChatInput } from "./input/ai_chat_input";
import { HistoryFetcher } from "./chat_page_context/history_fetcher";
import { ChatSideSheet } from "./secondary_sheet/chat_secondary_panel";

const MotionButton = motion.create(Button);

interface EventProps {
  has_messages: boolean;
}

declare global {
  interface WindowEventMap {
    "set-ai-message-count": CustomEvent<EventProps>;
    "set-chat-entrance-settled": CustomEvent<{ chat_id: string }>;
  }
}

export const GeneralAIChatPageInner = ({
  children,
  sheet,
}: {
  children: ReactNode;
  sheet: ReactNode;
}): ReactNode => {
  const { hasMessages, messages, response } = useChatPageContext();
  const { sheetOpen } = useChatPageContext();
  const dispatch = useChatPageDispatch();
  return (
    <div className="w-full h-screen max-h-screen px-4">
      <motion.div
        className="absolute top-4 right-4 z-10 opacity-90 hover:opacity-100 hover:bg-secondary rounded cursor-pointer"
        whileHover={{
          backgroundColor: "hsl(var(--secondary))",
        }}
      >
        <MotionButton
          transitionAll={false}
          key="general-ai-search"
          size={messages.length || sheetOpen ? "icon-xs" : "icon-lg"}
          variant="secondary"
          onClick={() => {
            dispatch({
              type: "set-sheet-open",
              payload: !sheetOpen,
            });
          }}
          initial={"hide"}
          variants={{
            hide: {
              x: 50,
              opacity: 0,
            },
            small: {
              x: 0,
              opacity: 1,
              width: 24,
              height: 24,
            },
            large: {
              x: 0,
              opacity: 1,
              width: 40,
              height: 40,
            },
          }}
          animate={messages.length || sheetOpen ? "small" : "large"}
          exit={"hide"}
        >
          <SearchIcon />
        </MotionButton>
      </motion.div>
      <div
        className={cn(
          "@container/chat mx-auto w-270 max-w-[calc(100%-2rem)] max-h-screen min-h-screen flex flex-col justify-between items-center",
        )}
      >
        <motion.div
          className={cn(
            "grow overflow-x-hidden overflow-y-auto w-[calc(100%+0.5rem)] translate-x-1 no-scrollbar flex flex-col",
            response === null && !hasMessages
              ? "showing-empty-chat justify-center items-center"
              : "justify-end items-end pb-4",
          )}
        >
          {children}
          {response === null && !messages.length ? (
            <EmptyChat />
          ) : (
            <CurrentlyStreamingMessage />
          )}
        </motion.div>
        <div className="relative h-0! w-full overflow-visible">
          <div
            className="w-full h-4 absolute bottom-0"
            style={{
              background:
                "linear-gradient(hsl(var(--background)/0.1), hsl(var(--background)))",
            }}
          />
        </div>
        <ChatInput />
      </div>
      {sheet}
    </div>
  );
};

GeneralAIChatPageInner.displayName = "GeneralAIChatPage";

export const GeneralAIChatPage = () => {
  return (
    <GeneralAIChatPageInner sheet={<ChatSideSheet />}>
      <ChatContent />
      <HistoryFetcher />
    </GeneralAIChatPageInner>
  );
};
