import React, { type ReactNode, useState } from "react";
import { AnimatePresence } from "framer-motion";
import { WelcomeToConundrum } from "./onboarding_sections/welcome/welcome_to_cdrm";
import { CreateTablesOnboardingPage } from "./onboarding_sections/create_tables/create_tables";
import { CreateToolIndexOnboardingSection } from "./onboarding_sections/create_tool_index/create_tool_index_section";
import { OnboardingChecklist } from "./onboarding_checklist/onboading_checklist";
import MediaQuery from "react-responsive";

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
    return (
        <div className="w-full h-full min-h-screen flex flex-col justify-center items-center min-[768px]:grid min-[768px]:grid-cols-[auto_1fr] min-[768px]:gap-x-6 text-foreground">
            <MediaQuery minWidth={768}>
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
                            body: "Build a local vector store with one click..."
                        },
                        {
                            id: "create-tool-index",
                            status: pendingFromStepIndex(2, onboardingSection),
                            label: "Seed Tool Index",
                            body: "Conundrum keeps your tool definitions in vector space so you never run up against context limits"
                        },
                    ]}
                />
            </MediaQuery>
            <div className="@container/onboarding flex flex-col justify-center items-center w-full h-fit @3xl:pr-6">
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
                        ) : (
                            <CreateToolIndexOnboardingSection
                                key="create-tool-index"
                                next={() => {
                                    setOnboardingSection(3);
                                }}
                                back={() => {
                                    setOnboardingSection(1);
                                }}
                            />
                        )}
                    </>
                </AnimatePresence>
            </div>
        </div>
    );
};

OnboardingPage.displayName = "OnboardingPage";
