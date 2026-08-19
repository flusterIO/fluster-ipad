import React, { type ReactNode } from "react";
import { OnboardingStep, type OnboardingStepProps } from "./onboarding_step";
import { motion } from "framer-motion";
import { OnboardingChecklistComplete } from "./onboarding_checklist_complete";
import { Button, buttonVariants } from "@/components/shad/button";
import { Link } from "react-router";
import { AppPaths } from "#/navigation/app_paths";

interface OnboardingChecklistProps {
    steps: OnboardingStepProps[];
    lastPage?: boolean
}

export const OnboardingChecklist = ({
    steps,
    lastPage
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
            {lastPage ? (<motion.div
                className="overflow-hidden"
                initial={{
                    height: 0
                }}
                animate={{
                    height: "auto"
                }}
            >
                <Link to={AppPaths.dashboard} className={buttonVariants()}>Take Me Home</Link>
            </motion.div>) : null}
        </motion.div>
    );
};

OnboardingChecklist.displayName = "OnboardingChecklist";
