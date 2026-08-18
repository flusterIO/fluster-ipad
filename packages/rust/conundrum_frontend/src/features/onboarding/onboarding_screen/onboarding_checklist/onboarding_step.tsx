import { cn } from "@/utils/shad_utils";
import { CheckIcon as CI } from "lucide-react";
import { AnimatePresence, motion } from "framer-motion";
import React, { type ReactNode } from "react";
import { RingLoader as RL } from "react-spinners";

const RingLoader = motion.create(RL);
const CheckIcon = motion.create(CI);

export interface OnboardingStepProps {
    label: ReactNode;
    body: ReactNode;
    status: "in-progress" | "complete" | "pending";
    id: string;
}

export const OnboardingStep = ({
    label,
    body,
    status,
    id
}: OnboardingStepProps): ReactNode => {
    const color =
        status === "in-progress"
            ? "bg-primary text-primary-foreground border-primary/50!"
            : status === "complete"
                ? "text-green-500 bg-secondary border border-green-500/50!"
                : "bg-secondary border border-primary/50!";
    return (
        <div className="grid grid-cols-[auto_1fr] gap-x-3">
            <div className={cn("w-5 h-5 grid place-items-center mt-2 relative transition-all duration-300", color, status === "in-progress" ? "rounded-full" : "rounded")}>
                <AnimatePresence key={id}>
                    {status === "in-progress" ? (
                        <RingLoader
                            initial={{
                                /* y: 10, */
                                scale: 0
                            }}
                            exit={{
                                /* y: -10, */
                                scale: 0
                            }}
                            animate={{
                                /* y: 0, */
                                scale: 1
                            }}
                            className="w-4 h-4 max-w-4 max-h-4 relative transition-all duration-300" size={16}
                        />
                    ) : status === "complete" ? (
                        <CheckIcon
                            className="absolute -top-1 -right-1.5"
                            initial={{
                                y: -10,
                                scale: 0
                            }}
                            exit={{
                                y: 10,
                                scale: 0
                            }}
                            animate={{
                                y: 0,
                                scale: 1
                            }}
                            transition={{
                                bounce: 0
                            }}
                        />
                    ) : (
                        <motion.div
                            className="place-self-center leading-0"
                            initial={{
                                y: -50,
                                scale: 0
                            }}
                            exit={{
                                y: 50,
                                scale: 0
                            }}
                            animate={{
                                y: 0,
                                scale: 1
                            }}
                        >
                            {"-"}
                        </motion.div>
                    )}
                </AnimatePresence>
            </div>
            <div className="flex flex-col justify-center items-start">
                <h5 className="text font-semibold text-fd-card-foreground">{label}</h5>
                <div className="text-sm text-fd-card-foreground/80">{body}</div>
            </div>
        </div>
    );
};

OnboardingStep.displayName = "OnboardingStep";
