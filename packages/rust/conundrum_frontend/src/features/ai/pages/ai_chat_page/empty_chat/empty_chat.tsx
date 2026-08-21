import React, { type ReactNode } from "react";
import { motion } from "framer-motion";

export const EmptyChat = (): ReactNode => {
    return (
        <div className="w-full h-full flex flex-col justify-center items-center grow text-foreground">
            <motion.h3
                className="text-xl @[350px]/chat:text-3xl font-bold"
                initial={{
                    opacity: 0,
                    scale: 0,
                }}
                animate={{
                    opacity: 1,
                    scale: 1,
                }}
                exit={{
                    opacity: 0,
                    scale: 0,
                }}
            >
                Lets change the world
            </motion.h3>
            <motion.div
                className="text-sm @[350px]/chat:text-base text-foreground/80"
                initial={{
                    opacity: 0,
                    scale: 0,
                }}
                animate={{
                    opacity: 1,
                    scale: 1,
                }}
                exit={{
                    opacity: 0,
                    scale: 0,
                }}
            >
                But we can start small. What are we working on today?
            </motion.div>
        </div>
    );
};

EmptyChat.displayName = "EmptyChat";
