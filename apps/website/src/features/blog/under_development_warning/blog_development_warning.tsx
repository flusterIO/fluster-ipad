"use client";
import { Button } from "#/core/shad/ui/button";
import { ConstructionIcon } from "lucide-react";
import React, { useEffect, useState, type ReactNode } from "react";
import { motion } from "framer-motion";

export const BlogDevelopmentWarning = (): ReactNode => {
    const [open, setOpen] = useState(false);
    const k = "have-shown-blog-dev-warn";
    useEffect(() => {
        const hasSet = window.localStorage.getItem(k) === "true";
        if (hasSet) {
            return;
        } else {
            setOpen(true);
        }
    }, []);
    const handleClose = (): void => {
        window.localStorage.setItem(k, "true");
        setOpen(false);
    };
    return (
        <motion.div
            className="fixed top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] max-w-[min(768px,90vw)] max-h-[min(540px,90vh)] bg-card text-fd-card-foreground border px-3 py-4 z-100"
            animate={open ? "open" : "close"}
            initial={"close"}
            variants={{
                open: {
                    scale: 1,
                    opacity: 1,
                },
                close: {
                    scale: 0,
                    opacity: 0,
                },
            }}
        >
            <div className="text-lg font-semibold">
                <div className="bg-orange-400 text-white w-fit h-fit p-2 rounded grid place-items-center inline mr-3">
                    <ConstructionIcon className="w-4 h-4 inline text-background" />
                </div>
                <div className="inline">Under active development</div>
            </div>
            <div className="max-w-full mt-3">
                <p>
                    This blog is being built along with a blogging framework for the
                    entire <span className="text-bold">Conundrum</span> ecosystem.
                </p>
                <p className="mt-2">
                    This is taking significantly more time than building a simple blogging
                    website, but when this is complete in a couple months, all users will
                    be able to publish their notes as a blog, independent of any specific
                    service, <span className="italic">including</span> Fluster.
                </p>
            </div>
            <div className="w-full flex flex-row justify-end items-center">
                <Button
                    onClick={handleClose}
                    className="bg-orange-400 hover:bg-orange-500 text-background"
                >
                    Don't show me again
                </Button>
            </div>
        </motion.div>
    );
};

BlogDevelopmentWarning.displayName = "BlogDevelopmentWarning";
