import React, { type ReactNode } from "react";
import { OnboardingStep, type OnboardingStepProps } from "./onboarding_step";
import { motion } from "framer-motion";

interface OnboardingChecklistProps {
    steps: OnboardingStepProps[];
}

export const OnboardingChecklist = ({
    steps,
}: OnboardingChecklistProps): ReactNode => {
    return (
        <motion.div
            className="w-[min(300px,25vw)] px-3 py-4 h-screen flex flex-col justify-center items-center border-r bg-fd-card space-y-4"
            initial={{
                x: "-100%",
                opacity: 0
            }}
            animate={{
                x: 0,
                opacity: 1
            }}
            exit={{
                x: "-100%",
                opacity: 0
            }}
        >
            {steps.map((s) => {
                return <OnboardingStep {...s} key={s.id} />;
            })}
        </motion.div>
    );
};

OnboardingChecklist.displayName = "OnboardingChecklist";
