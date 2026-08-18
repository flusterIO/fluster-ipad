import React, { type ReactNode } from "react";
import { type OnboardingSectionProps } from "../welcome/welcome_to_cdrm";
import { Button } from "@/components/shad/button";
import { motion } from "framer-motion";
import { ToolCaseIcon } from "lucide-react";

export const CreateToolIndexOnboardingSection = ({
    next,
    back,
}: OnboardingSectionProps): ReactNode => {
    return (
        <motion.div
            initial={{
                opacity: 0,
            }}
            animate={{
                opacity: 1,
            }}
            exit={{
                opacity: 0,
            }}
            className="max-w-[min(90%,640px)] flex flex-col justify-center items-start bg-fd-card text-fd-card-foreground rounded-xl border p-4 gap-y-3"
        >
            <div className="grid grid-cols-[auto_1fr] place-items-center gap-x-2">
                <motion.div
                    initial={{
                        scale: 0,
                        opacity: 0,
                        rotateZ: -360,
                    }}
                    animate={{
                        scale: 1,
                        opacity: 1,
                        rotateZ: 0,
                    }}
                    className="bg-primary text-foreground rounded-full p-2"
                >
                    <ToolCaseIcon className="w-6 h-6" />
                </motion.div>
                <motion.h2
                    initial={{
                        opacity: 0,
                    }}
                    animate={{
                        opacity: 1,
                    }}
                    exit={{
                        opacity: 0,
                        x: 100,
                    }}
                    className="font-bold text-2xl lg:text-3xl"
                >
                    Dynamic Tool Index
                </motion.h2>
            </div>
            <motion.p
                initial={{
                    opacity: 0,
                }}
                animate={{
                    opacity: 1,
                }}
                exit={{
                    x: -100,
                    opacity: 0,
                }}
            >
                Conundrum includes a complete <span className="font-bold">MCP</span>{" "}
                server with a growing list of tools to help make your data accessible to
                AI.
            </motion.p>
            <motion.p
                initial={{
                    opacity: 0,
                }}
                animate={{
                    opacity: 1,
                }}
                exit={{
                    x: -100,
                    opacity: 0,
                }}
            >
                To get around the limitations regarding the number of tools a model can
                handle before hallucinating, Conundrum stores each tool definition in
                the model's vector space for dynamic retrieval. Click below to generate
                this index now.
            </motion.p>
            <div className="w-full flex flex-row justify-end items-center gap-x-4">
                <Button variant={"outline"} onClick={back}>
                    Back
                </Button>
                <Button onClick={next}>Generate Index</Button>
            </div>
        </motion.div>
    );
};

CreateToolIndexOnboardingSection.displayName = "CreateTablesOnboardingPage";
