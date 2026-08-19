import React, { type ReactNode } from "react";
import { type OnboardingSectionProps } from "../welcome/welcome_to_cdrm";
import { Button } from "@/components/shad/button";
import { motion } from "framer-motion";
import { DatabaseIcon } from "lucide-react";
import { rspc } from "@/app/rspc_client";
import consola from "consola";
import { SomeTablesExistWarning } from "./some_tables_exist";
import { useLogger } from "#/logging/state/hooks/use_logger";

export const CreateTablesOnboardingPage = ({
    next,
    back,
    setResults,
}: OnboardingSectionProps): ReactNode => {
    const { data: status } = rspc.useQuery(["backend_status", null]);
    const { mutateAsync } = rspc.useMutation("initialize.step_1_init_db");
    const log = useLogger();
    const handleNext = async (partialTables: boolean): Promise<void> => {
        try {
            await mutateAsync(
                {},
                {
                    onError: (e) => {
                        consola.error("Error: ", e);
                        log(
                            {
                                message: partialTables
                                    ? "Failed to generate the database successfully as the database was already partially generated."
                                    : "Failed to successfuly generate the database.",
                                title: "Database Failure",
                                ai_description: partialTables
                                    ? "The user attempted to generate a LanceDB database on their system but failed because the database was already partially generated. This is located in the user's default data directory under the `/conundrum/data/database` subpath."
                                    : "The user attempted to generate a LanceDB database on their system but failed for unknown reasons. This is located in the user's default data directory under the `/conundrum/data/database` subpath.",
                                purpose: "process-complete",
                                severity: "warning",
                            },
                            true,
                        ).catch((err: unknown) => {
                            consola.error("Error: ", err);
                        });
                    },
                    onSuccess: (data) => {
                        setResults(data);
                        next();
                        log({
                            message: "Successfully generated a local LanceDB vector store.",
                            title: "Database Generated Successfully",
                            ai_description:
                                "The user successfully generated a LanceDB vector database in their operating system's default data directory.",
                            purpose: "process-complete",
                            severity: "success",
                        }).catch((err: unknown) => {
                            consola.error("Error: ", err);
                        });
                    },
                },
            );
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
                    <DatabaseIcon className="w-6 h-6" />
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
                    Local Vector Store
                </motion.h2>
            </div>
            {status?.any_tables_exist ? (
                <SomeTablesExistWarning />
            ) : (
                <>
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
                        Conundrum integrates with a <span className="font-bold">100%</span>{" "}
                        local vector store that keeps your notes on your own machine and the
                        cost of operation down.
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
                        Clicking below will generate a{" "}
                        <span className="italic font-bold">LanceDB</span> database in your
                        operating system's default data directory, with dozens of
                        graph-oriented models ready to go.
                    </motion.p>
                </>
            )}
            <div className="w-full flex flex-row justify-end items-center gap-x-4">
                <Button variant={"outline"} onClick={back}>
                    Back
                </Button>
                <Button
                    onClick={() => {
                        handleNext(Boolean(status?.any_tables_exist)).catch(
                            (err: unknown) => {
                                consola.error("Error: ", err);
                            },
                        );
                    }}
                >
                    Create Database
                </Button>
            </div>
        </motion.div>
    );
};

CreateTablesOnboardingPage.displayName = "CreateTablesOnboardingPage";
