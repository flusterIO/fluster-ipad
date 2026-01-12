import { buttonVariants, Button } from "@/shared_components/shad/button";
import { H1 } from "@/shared_components/typography/typography";
import { openUrl } from "@tauri-apps/plugin-opener";
import React, { type ReactNode } from "react";
import { useNavigate } from "react-router";
import { incrementOnboardingPageIndex } from "../../state/actions/onboarding_index_utils";
import { useOnboardingStateContext, useOnboardingStateDispatch } from "../../state/onboarding_context";

export const OnboardingNotifyOfModelsDownloading = (): ReactNode => {
    const state = useOnboardingStateContext();
    const dispatch = useOnboardingStateDispatch();
    const nav = useNavigate();
    const handleClick = (): void => {
        incrementOnboardingPageIndex(state.pageIndex, dispatch, nav);
    };

    const openOllamaDocs = (): void => {
        openUrl("https://ollama.com/download");
    };
    return (
        <div className="max-w-[768px] flex flex-col justify-center items-start gap-8 px-8">
            <H1>Local AI</H1>
            <p className="text-muted-foreground">
                Fluster uses Ollama under the hood to manage local AI models. While
                Fluster can be used without this AI functionality, many tasks and more
                advanced features will be non-functional.
            </p>
            <p className="text-muted-foreground">
                If you do not already have Ollama installed, please click below to find
                the proper app for your operating system.
            </p>
            <div className="w-full flex flex-row justify-end items-center gap-4">
                <a
                    role="button"
                    className={buttonVariants({ variant: "outline" })}
                    onClick={openOllamaDocs}
                >
                    Install Ollama
                </a>
                <Button onClick={handleClick}>I'm aware</Button>
            </div>
        </div>
    );
};

OnboardingNotifyOfModelsDownloading.displayName =
    "OnboardingNotifyOfModelsDownloading";
