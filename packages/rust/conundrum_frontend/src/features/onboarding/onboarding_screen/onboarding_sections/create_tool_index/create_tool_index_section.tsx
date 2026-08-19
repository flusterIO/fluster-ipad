import React, { type ReactNode } from "react";
import { type OnboardingSectionProps } from "../welcome/welcome_to_cdrm";
import { Button } from "@/components/shad/button";
import { motion } from "framer-motion";
import { ToolCaseIcon } from "lucide-react";
import { rspc } from "@/app/rspc_client";
import consola from "consola";
import { useLogger } from "#/logging/state/hooks/use_logger";

export const CreateToolIndexOnboardingSection = ({
    next,
    back,
    setResults,
}: OnboardingSectionProps): ReactNode => {
    const { mutateAsync } = rspc.useMutation("initialize.step_2_init_tool_index");
    const logger = useLogger();
    const handleNext = async (): Promise<void> => {
        try {
            const res = await mutateAsync(
                {},
                {
                    onError: (err) => {
                        consola.error("Database Error: ", err);
                        logger(
                            {
                                title: "Created Tool Index",
                                message:
                                    "Conundrum successfully seeded the dynamic tool index. Your AI now has access to a growing list of tools to help you accomplish all of your academic goals.",
                                ai_description:
                                    "The user successfully seeded their database with all of the tools you now have access to.",
                                purpose: "process-complete",
                                severity: "success",
                            },
                            true,
                        ).catch((err: unknown) => {
                            consola.error("Error: ", err);
                        });
                    },
                    onSuccess: () => {
                        logger({
                            title: "Created Tool Index",
                            message:
                                "Conundrum successfully seeded the dynamic tool index. Your AI now has access to a growing list of tools to help you accomplish all of your academic goals.",
                            ai_description:
                                "The user successfully seeded their database with all of the tools you now have access to.",
                            purpose: "process-complete",
                            severity: "success",
                        }).catch((err: unknown) => {
                            consola.error("Error: ", err);
                        });
                    },
                },
            );
            setResults(res);
            next();
        } catch (err: unknown) {
            consola.error("Error: ", err);
        }
    };
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
                <Button
                    onClick={() => {
                        handleNext().catch((err: unknown) => {
                            consola.error("Error: ", err);
                        });
                    }}
                >
                    Generate Index
                </Button>
            </div>
        </motion.div>
    );
};

CreateToolIndexOnboardingSection.displayName = "CreateTablesOnboardingPage";
