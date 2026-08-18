import React, { useEffect, useState, type ReactNode } from "react";
import { motion } from "framer-motion";
import { WelcomeTitle } from "./welcome_title";
import { Button } from "@/components/shad/button";
import { DatabaseIcon } from "lucide-react";

const MotionButton = motion.create(Button);

export interface OnboardingSectionProps {
    next: () => void;
    back: () => void;
}

export const WelcomeToConundrum = ({
    next,
}: Omit<OnboardingSectionProps, "back">): ReactNode => {

    return (
        <motion.div className="max-w-3xl flex flex-col justify-start items-start gap-y-3 gap-x-4 @lg/onboarding:grid @lg/onboarding:grid-cols-[auto_200px] border rounded p-4 bg-fd-card place-items-center"
            initial={{
                opacity: 0,
            }}
            animate={{
                opacity: 1,
            }}
            exit={{
                opacity: 0,
            }}
        >
            <div className="w-full h-fit flex flex-col justify-start items-start">
                <h1 className="text-4xl font-semibold">
                    Welcome to <span className="font-bold">Conundrum</span>
                </h1>
                <div
                    key={"subtitle"}
                    className="text-foreground/60 max-w-[min(350px,90%)] text-left mb-4"
                >
                    Your database is missing, which probably means we're just setting
                    things up.
                </div>
                <div
                    className="text-sm"
                >
                    We have to build your local database, but in the end, you'll have a fully working modular toolkit powered by <span className="font-bold">LanceDB</span> and <a href="https://flusterapp.com" className="font-bold">Conundrum</a>, ready to tackle of your academic goals.
                </div>
            </div>
            <div className="flex flex-col justify-center items-center @lg/onboarding:h-full">
                <MotionButton
                    onClick={next}
                    size="lg"
                    className="text-xl"
                >
                    <DatabaseIcon className="w-6 h-6" />
                    <span>Set me up</span>
                </MotionButton>
            </div>
        </motion.div>
    );
};

WelcomeToConundrum.displayName = "WelcomeToConundrum";
