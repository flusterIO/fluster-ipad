import React, { type ReactNode, useState } from "react";
import { AnimatePresence } from "framer-motion";
import { WelcomeToConundrum } from "./onboarding_sections/welcome/welcome_to_cdrm";
import { CreateTablesOnboardingPage } from "./onboarding_sections/create_tables/create_tables";
import { CreateToolIndexOnboardingSection } from "./onboarding_sections/create_tool_index/create_tool_index_section";
import { OnboardingChecklist } from "./onboarding_checklist/onboading_checklist";
import { useMediaQuery } from "react-responsive";
import { OnboardingWrapup } from "./onboarding_wrapup/onboarding_wrapup";
import { cn } from "@/utils/shad_utils";

const pendingFromStepIndex = (
    idx: number,
    activeIdx: number,
): "pending" | "in-progress" | "complete" => {
    if (idx < activeIdx) {
        return "complete";
    }
    if (idx > activeIdx) {
        return "pending";
    }
    return "in-progress";
};

export const OnboardingPage = (): ReactNode => {
    const [onboardingSection, setOnboardingSection] = useState(0);
    const mq = useMediaQuery({
        minWidth: 768
    })
    const showSide = (onboardingSection <= 2) && mq;
    return (
        <div className={cn("w-full h-full min-h-screen text-foreground flex flex-col justify-center items-center", showSide && "min-[768px]:grid min-[768px]:grid-cols-[auto_1fr] min-[768px]:gap-x-6")}>
            <AnimatePresence key={onboardingSection <= 2 ? "onboarding-show" : "onboarding-hide"}>
                {onboardingSection <= 2 ? (
                    <OnboardingChecklist
                        steps={[
                            {
                                id: "welcome",
                                status: pendingFromStepIndex(0, onboardingSection),
                                label: "Get Started With Conundrum",
                                body: "Just a quick intro so you know what exactly's going on with your computer",
                            },
                            {
                                id: "create-db",
                                status: pendingFromStepIndex(1, onboardingSection),
                                label: "Create Database",
                                body: "Build a local vector store with one click."
                            },
                            {
                                id: "create-tool-index",
                                status: pendingFromStepIndex(2, onboardingSection),
                                label: "Seed Tool Index",
                                body: "Your model has choices."
                            },
                        ]}
                    />
                ) : null
                }
            </AnimatePresence>
            <div className="@container/onboarding flex flex-col justify-center items-center w-full h-fit @max-3xl:px-6 @3xl:pr-6">
                <AnimatePresence key={onboardingSection}>
                    <>
                        {onboardingSection === 0 ? (
                            <WelcomeToConundrum
                                key="welcome"
                                next={() => {
                                    setOnboardingSection(1);
                                }}
                            />
                        ) : onboardingSection === 1 ? (
                            <CreateTablesOnboardingPage
                                key="create-tables"
                                next={() => {
                                    setOnboardingSection(2);
                                }}
                                back={() => {
                                    setOnboardingSection(0);
                                }}
                            />
                        ) : onboardingSection === 2 ? (
                            <CreateToolIndexOnboardingSection
                                key="create-tool-index"
                                next={() => {
                                    setOnboardingSection(3);
                                }}
                                back={() => {
                                    setOnboardingSection(1);
                                }}
                            />
                        ) : (
                            <OnboardingWrapup />
                        )}
                    </>
                </AnimatePresence>
            </div>
        </div>
    );
};

OnboardingPage.displayName = "OnboardingPage";
