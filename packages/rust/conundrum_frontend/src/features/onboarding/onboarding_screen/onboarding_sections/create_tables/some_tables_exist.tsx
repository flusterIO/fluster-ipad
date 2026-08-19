import { PlainInlineCode } from "#/ui/typography/inline_code";
import { rspc } from "@/app/rspc_client";
import React, { useEffect, useState, type ReactNode } from "react";
import { MissingTable } from "./missing_tables";
import { motion } from "framer-motion";
import { ChevronLeft, ChevronRight } from "lucide-react";
import { cn } from "@/utils/shad_utils";

export const SomeTablesExistWarning = (): ReactNode => {
    const [activeHeight, setActiveHeight] = useState(0);
    const [focusedIndex, setFocusedIndex] = useState(0);
    const [isInitial, setIsInitial] = useState(true);
    const { data: health } = rspc.useQuery(["rpc_health", null]);
    useEffect(() => {
        setFocusedIndex(0);
    }, [health]);
    return (
        <div className="indent-4">
            <p>
                It looks like <span className="italic">some</span> of your tables
                already exist. This usually means that something went wrong, but if you
                would like to attempt to re-generate the database, click the button
                below.
            </p>
            <p>
                This won't overwrite anything, but will re-generate any specific missing
                tables. If you would like start with a fresh database, delete the{" "}
                <PlainInlineCode color="code" code="conundrum/data/database" /> folder
                in your operating system's default data directory.
            </p>
            <h5 className="text-lg font-bold mt-4 mb-0">Missing Tables</h5>
            <div className="text-sm text-foreground/70! leading-0 mt-1">
                With the description that AI sees.
            </div>
            <motion.div
                className="w-full grid grid-cols-[48px_1fr_48px] transition-all duration-300 my-4"
                style={{
                    height: `${activeHeight}px`,
                }}
            >
                <div
                    onClick={() => {
                        const n =
                            health?.table_reports.filter((f) => !f.exists).length ?? 0;
                        setIsInitial(false);
                        setFocusedIndex(focusedIndex >= 1 ? focusedIndex - 1 : n - 1);
                    }}
                    className="flex flex-col justify-center items-center bg-secondary/50 text-secondary-foreground"
                >
                    <ChevronLeft />
                </div>
                <motion.div className="w-full h-full flex flex-start justify-start max-w-full relative overflow-hidden">
                    {health?.table_reports
                        .filter((x) => !x.exists)
                        .map((t, i) => {
                            return (
                                <MissingTable
                                    key={t.description.table}
                                    style={{
                                        transform: `translateX(${(i - focusedIndex) * 100}%)`,
                                    }}
                                    setActiveHeight={setActiveHeight}
                                    active={focusedIndex === i}
                                    className={cn(
                                        "absolute max-w-full left-0 top-0",
                                        isInitial && "h-fit",
                                    )}
                                    table={t}
                                />
                            );
                        })}
                </motion.div>
                <div
                    onClick={() => {
                        const n =
                            health?.table_reports.filter((f) => !f.exists).length ?? 1;
                        setFocusedIndex((focusedIndex + 1) % n);
                        setIsInitial(false);
                    }}
                    className="bg-secondary/50 text-secondary-foreground flex flex-col justify-center items-center"
                >
                    <ChevronRight />
                </div>
            </motion.div>
        </div>
    );
};

SomeTablesExistWarning.displayName = "SomeTablesExistWarning";
