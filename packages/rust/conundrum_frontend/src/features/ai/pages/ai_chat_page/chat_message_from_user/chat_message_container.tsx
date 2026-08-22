import React, { type ReactNode } from "react";
import { motion } from "framer-motion";
import { useSearchParams } from "react-router";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/shad/tooltip";
import { DateTimeComponent } from "#/datetime/components/date_time";

interface ChatMessageContainerProps {
    index: number;
    children: ReactNode;
    className: string;
    isLast: boolean;
    ctime?: string | Date
}


const CM = ({
    index,
    className,
    children,
    isLast,
}: ChatMessageContainerProps & { className?: string }): ReactNode => {
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
    )
}

export const ChatMessageContainer = (props: ChatMessageContainerProps): ReactNode => {
    if (props.ctime) {
        const { className, ..._props } = props;
        return (
            <Tooltip>
                <TooltipTrigger className={className} >
                    <CM {..._props} />
                </TooltipTrigger>
                <TooltipContent>
                    <DateTimeComponent dateTime={props.ctime} format="full-with-time" />
                </TooltipContent>
            </Tooltip>
        )
    } else {
        return (
            <CM {...props} />
        )
    }
};

ChatMessageContainer.displayName = "ChatMessageContainer";
