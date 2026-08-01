import React, { type ReactNode } from "react";
import { motion } from "framer-motion";

interface EmptyCardDataTextProps {
    children: ReactNode;
}

export const EmptyCardDataText = ({
    children,
}: EmptyCardDataTextProps): ReactNode => {
    return (
        <motion.div
            className="w-full text-center h-fit @xl/main:h-full @xl/main:flex flex-col justify-center items-center text-lg font-semibold text-fd-card-foreground/50"
            initial={{
                scale: 0,
                opacity: 0,
            }}
            animate={{
                scale: 1,
                opacity: 1,
            }}
        >
            {children}
        </motion.div>
    );
};

EmptyCardDataText.displayName = "EmptyCardDataText";
