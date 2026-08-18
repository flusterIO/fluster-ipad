import React, { type ReactNode } from "react";
import { OnboardingStep, type OnboardingStepProps } from "./onboarding_step";

interface OnboardingChecklistProps {
    steps: OnboardingStepProps[];
}

export const OnboardingChecklist = ({
    steps,
}: OnboardingChecklistProps): ReactNode => {
    return (
        <div className="w-[min(300px,25vw)] px-3 py-4 h-screen flex flex-col justify-center items-center border-r bg-fd-card space-y-4">
            {steps.map((s) => {
                return <OnboardingStep {...s} key={s.id} />;
            })}
        </div>
    );
};

OnboardingChecklist.displayName = "OnboardingChecklist";
