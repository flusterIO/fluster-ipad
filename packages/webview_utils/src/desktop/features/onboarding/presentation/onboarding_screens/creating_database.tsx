import { Button } from "@/shared_components/shad/button";
import { H1 } from "@/shared_components/typography/typography";
import { commands } from "@fluster/desktop_bindings";
import React, { type ReactNode } from "react";
import { useNavigate } from "react-router";
import { incrementOnboardingPageIndex } from "../../state/actions/onboarding_index_utils";
import { useOnboardingStateContext, useOnboardingStateDispatch } from "../../state/onboarding_context";

const OnboardingCreateDatabaseScreen = (): ReactNode => {
    const state = useOnboardingStateContext();
    const dispatch = useOnboardingStateDispatch();
    const nav = useNavigate();
    const handleClick = async (): Promise<void> => {
        /* const res = await commands.initializeDatabase(); */
        const res = { status: "error" }
        if (res.status === "error") {
            console.warn(`Fluster encountered an error while attempting to initialize your database.
If you've already set up Fluster initially, this may just mean that the tables already exist and you can safely ignore this.
If you do encounter issues with your database, please submit an issue on Github.`);
        }
        incrementOnboardingPageIndex(state.pageIndex, dispatch, nav);
    };
    return (
        <div className="max-w-[768px] flex flex-col justify-center items-start gap-8 px-8">
            <H1>Welcome to Fluster!</H1>
            <p className="text-muted-foreground">
                It looks like you're either new here or resetting your database. Don't
                worry, we just have to regenerate some tables.
            </p>
            <div className="w-full flex flex-row justify-end items-center">
                <Button onClick={handleClick}>Create Database</Button>
            </div>
        </div>
    );
};

OnboardingCreateDatabaseScreen.displayName = "OnboardingCreateDatabaseScreen";

export default OnboardingCreateDatabaseScreen;
