import React, { type ReactNode, useState } from "react";
import { AnimatePresence } from "framer-motion";
import { WelcomeToConundrum } from "./onboarding_sections/welcome/welcome_to_cdrm";
import { CreateTablesOnboardingPage } from "./onboarding_sections/create_tables/create_tables";
import { CreateToolIndexOnboardingSection } from "./onboarding_sections/create_tool_index/create_tool_index_section";

export const OnboardingPage = (): ReactNode => {
    const [onboardingSection, setOnboardingSection] = useState(0);
    return (
        <div className="w-full h-full min-h-screen flex flex-col justify-center items-center text-foreground">
            <AnimatePresence key={onboardingSection}>
                <div className="flex flex-col justify-center items-center w-fit h-fit">
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
                </div>
            </AnimatePresence>
        </div>
    );
};

OnboardingPage.displayName = "OnboardingPage";
