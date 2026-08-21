import React, { type ReactNode } from "react";
import { motion } from "framer-motion";
import consola from "consola";
import { useSearchParams } from "react-router";

interface ChatMessageContainerProps {
    index: number;
    children: ReactNode;
    className: string;
    isLast: boolean;
}

export const ChatMessageContainer = ({
    index,
    className,
    children,
    isLast,
}: ChatMessageContainerProps): ReactNode => {
    const [sp] = useSearchParams();
    const convo_id = sp.get("convo");
    return (
        <motion.div
            onAnimationComplete={() => {
                if (isLast) {
                    window.dispatchEvent(
                        new CustomEvent("set-chat-entrance-settled", {
                            detail: {
                                convo_id,
                            },
                        }),
                    );
                }
            }}
            className={className}
            initial={{
                scale: 0,
            }}
            animate={{
                scale: 1,
            }}
            exit={{
                x: 100,
                scale: 0,
            }}
            transition={{
                delay: index * 0.05,
            }}
        >
            {children}
        </motion.div>
    );
};

ChatMessageContainer.displayName = "ChatMessageContainer";
