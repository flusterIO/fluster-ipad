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
    const [showSubtitle, setShowSubtitle] = useState(false);
    const [haveSetSubtitle, setHaveSetSubtitle] = useState(false);

    useEffect(() => {
        if (!haveSetSubtitle && !showSubtitle) {
            setHaveSetSubtitle(true);
            setTimeout(() => {
                setShowSubtitle(true);
            }, 2000);
        }
    }, [haveSetSubtitle, showSubtitle]);

    useEffect(() => {
        console.log("showSubtitle: ", showSubtitle);
    }, [showSubtitle]);

    return (
        <>
            <WelcomeTitle />
            <motion.div
                key={"subtitle"}
                initial={"hide"}
                animate={showSubtitle ? "show" : "hide"}
                variants={{
                    hide: {
                        x: -100,
                        opacity: 0,
                    },
                    show: {
                        x: 0,
                        opacity: 1,
                    },
                }}
                exit={{
                    x: 100,
                    opacity: 0,
                }}
                className="text-foreground/60"
            >
                You're missing a few things, so let's set them up...
            </motion.div>
            <div className="w-full flex flex-row justify-end items-center mt-3">
                <MotionButton
                    animate={showSubtitle ? "show" : "hide"}
                    initial={"hide"}
                    variants={{
                        hide: {
                            scale: 0,
                            opacity: 0,
                        },
                        show: {
                            scale: 1,
                            opacity: 1,
                        },
                    }}
                    exit={{
                        scale: 0,
                        opacity: 0,
                    }}
                    onClick={next}
                    size="lg"
                    className="text-xl"
                >
                    <DatabaseIcon className="w-6 h-6" />
                    <span>Set me up</span>
                </MotionButton>
            </div>
        </>
    );
};

WelcomeToConundrum.displayName = "WelcomeToConundrum";
