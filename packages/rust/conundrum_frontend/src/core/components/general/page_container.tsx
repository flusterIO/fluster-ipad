import { cn } from "@/utils/shad_utils";
import React, { type ReactNode } from "react";
import { motion } from "framer-motion";
import { PageTitleGroup } from "./page_title_group";

interface PageContainerProps {
    children: ReactNode;
    className?: string;
    title?: ReactNode;
    subtitle?: ReactNode;
    center?: boolean;
    toolbar?: ReactNode;
    toolbarClasses?: string;
    itemClasses?: string;
}

export const PageContainer = ({
    children,
    className,
    title,
    subtitle,
    center,
    toolbar,
    toolbarClasses,
    itemClasses,
}: PageContainerProps): ReactNode => {
    return (
        <div
            className={cn(
                "w-full h-fit min-h-[calc(100vh-4rem)] max-w-270 mx-auto py-8 px-4",
                center && "flex flex-col",
                className,
            )}
        >
            {title ? <PageTitleGroup title={title} subtitle={subtitle} /> : null}
            {toolbar ? (
                <div
                    className={cn(
                        "w-full flex flex-row justify-start items-center",
                        toolbarClasses,
                    )}
                >
                    {toolbar}
                </div>
            ) : null}
            <motion.div
                initial={{
                    opacity: 0,
                }}
                animate={{
                    opacity: 1,
                }}
                transition={{
                    delay: title && subtitle ? 0.2 : title ? 0.1 : 0,
                }}
                className={
                    center
                        ? "grow flex flex-col justify-center items-center text-foreground"
                        : cn("py-6 text-foreground", itemClasses)
                }
            >
                {children}
            </motion.div>
        </div>
    );
};

PageContainer.displayName = "PageContainer";
