import React, { useEffect, useState, type ReactNode } from "react";
import { motion } from "framer-motion";
import { useSearchParams } from "react-router";
import {
    Tooltip,
    TooltipContent,
    TooltipTrigger,
} from "@/components/shad/tooltip";
import { DateTimeComponent } from "#/datetime/components/date_time";

interface ChatMessageContainerProps {
    index: number;
    children: ReactNode;
    className?: string;
    isLast: boolean;
    ctime?: string | Date;
    containerClasses?: string;
}

const CM = ({
    index,
    className,
    children,
    isLast,
}: ChatMessageContainerProps & { className?: string }): ReactNode => {
    const [sp] = useSearchParams();
    const convo_id = sp.get("convo");
    const [haveAnimated, setHaveAnimated] = useState(false)
    return (
        <motion.div
            onAnimationComplete={() => {
                setHaveAnimated(true)
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
                opacity: 0,
            }}
            animate={{
                scale: 1,
                opacity: 1,
            }}
            exit={{
                /* x: 100, */
                /* scale: 0, */
                opacity: 0,
            }}
            transition={{
                delay: haveAnimated ? 0 : index * 0.05,
            }}
        >
            {children}
        </motion.div>
    );
};

export const ChatMessageContainer = (
    props: ChatMessageContainerProps,
): ReactNode => {
    if (props.ctime) {
        return (
            <Tooltip>
                <TooltipTrigger
                    render={<CM {...props} />}
                    className={props.containerClasses} />
                <TooltipContent>
                    <DateTimeComponent dateTime={props.ctime} format="full-with-time" />
                </TooltipContent>
            </Tooltip>
        );
    } else {
        return <CM {...props} />;
    }
};

ChatMessageContainer.displayName = "ChatMessageContainer";
