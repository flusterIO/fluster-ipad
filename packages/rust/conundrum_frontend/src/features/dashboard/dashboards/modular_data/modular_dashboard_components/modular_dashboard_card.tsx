import { cn } from "@/utils/shad_utils";
import React, { type ReactNode } from "react";
import { motion, type TargetAndTransition } from "framer-motion";

interface ModularDashboardCardProps {
    children: ReactNode;
    title?: ReactNode;
    desc?: ReactNode;
    center?: boolean;
    className?: string;
    initial?: TargetAndTransition;
    animate?: TargetAndTransition;
    exitAnim?: TargetAndTransition;
}

export const ModularDashboardCard = ({
    title,
    desc,
    children,
    center,
    className,
    initial,
    animate,
    exitAnim,
}: ModularDashboardCardProps): ReactNode => {
    return (
        <motion.div
            className={cn(
                "border rounded-xl bg-fd-card w-full h-full p-4",
                className,
            )}
            initial={
                initial ?? {
                    scale: 0,
                    opacity: 0,
                    origin: "center",
                }
            }
            animate={
                animate ?? {
                    scale: 1,
                    opacity: 1,
                    origin: "center",
                }
            }
            exit={
                exitAnim ?? {
                    scale: 0,
                    opacity: 0,
                    origin: "center",
                }
            }
        >
            {title ? (
                <>
                    <h5 className="text-sm font-semibold text-foreground/80">{title}</h5>
                    {desc ? <div>{desc}</div> : null}
                </>
            ) : null}
            {center ? (
                <div className="w-full h-full flex flex-col justify-center items-center p-4">
                    {children}
                </div>
            ) : null}
        </motion.div>
    );
};

ModularDashboardCard.displayName = "ModularDashboardCard";
