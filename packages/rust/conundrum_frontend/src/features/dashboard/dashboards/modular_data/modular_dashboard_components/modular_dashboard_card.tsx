import { cn } from "@/utils/shad_utils";
import React, { type ReactNode } from "react";
import { motion, type TargetAndTransition } from "framer-motion";
import { EllipsisVertical } from "lucide-react";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuGroup,
    DropdownMenuItem,
    DropdownMenuLabel,
    DropdownMenuTrigger,
} from "@/components/shad/dropdown-menu";

export interface CardOption {
    label: ReactNode;
    action: () => void;
}

interface ModularDashboardCardProps {
    children: ReactNode;
    title?: ReactNode;
    desc?: ReactNode;
    center?: boolean;
    className?: string;
    initial?: TargetAndTransition;
    animate?: TargetAndTransition;
    exitAnim?: TargetAndTransition;
    footer?: ReactNode;
    options?: Record<string, CardOption[]>;
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
    footer,
    options = {},
}: ModularDashboardCardProps): ReactNode => {
    return (
        <motion.div
            className={cn(
                "border rounded-xl bg-fd-card w-full h-full p-4 flex flex-col relative",
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
            {Object.keys(options).length ? (
                <DropdownMenu>
                    <DropdownMenuTrigger
                        render={(props) => {
                            return (
                                <EllipsisVertical
                                    className="text-foreground size-4 absolute top-2 right-2"
                                    {...props}
                                    onClick={(e) => {
                                        e.preventDefault();
                                        e.stopPropagation();
                                    }}
                                />
                            );
                        }}
                    />
                    <DropdownMenuContent>
                        {Object.keys(options).map((o) => {
                            const opts = options[o];
                            return (
                                <DropdownMenuGroup>
                                    <DropdownMenuLabel>{o}</DropdownMenuLabel>
                                    {opts.map((opt) => {
                                        return (
                                            <DropdownMenuItem onClick={opt.action}>
                                                {opt.label}
                                            </DropdownMenuItem>
                                        );
                                    })}
                                </DropdownMenuGroup>
                            );
                        })}
                    </DropdownMenuContent>
                </DropdownMenu>
            ) : null}
            {title ? (
                <>
                    <h5 className="text-sm font-semibold text-foreground/80">{title}</h5>
                    {desc ? <div>{desc}</div> : null}
                </>
            ) : null}
            {center ? (
                <div className="w-full h-full flex flex-col justify-center items-center p-4 grow">
                    {children}
                </div>
            ) : (
                children
            )}
            {footer ?? null}
        </motion.div>
    );
};

ModularDashboardCard.displayName = "ModularDashboardCard";
