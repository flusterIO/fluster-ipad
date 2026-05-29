import React, { useEffect, useRef, useState, type ReactNode } from "react";
import { type BlogSidebarProps } from "../blog_sidebar";
import { LucideFolderPen, LucidePaperclip } from "lucide-react";
import { subjectToUrl, topicToUrl } from "@conundrum/ts/pathUtils";
import { BLOGURL } from "../../constants";
import { SidebarMobileDrawerSection } from "./sidebar_mobile_drawer_section";
import { motion } from "framer-motion";



declare global {
    interface WindowEventMap {
        "toggle-drawer": CustomEvent<null>;
    }
}

export const SidebarMobileDrawer = (props: BlogSidebarProps): ReactNode => {
    const [open, setOpen] = useState(false);
    const openRef = useRef(open);
    const handleToggle = (): void => {
        openRef.current = !openRef.current;
        setOpen(!openRef.current);
    };
    useEffect(() => {
        window.addEventListener("toggle-drawer", handleToggle);
        return () => {
            window.removeEventListener("toggle-drawer", handleToggle);
        };
    }, []);


    return (
        <>
            <div />
            <motion.div
                className="w-full absolute bottom-0 top-0 left-0 right-0 origin-bottom z-10 bg-card text-card-foreground"
                initial={closed}
                animate={open ? "open" : "closed"}
                variants={{
                    open: {
                        height: "100vh",
                        opacity: 1
                    },
                    closed: {
                        height: 0,
                        opacity: 0
                    }
                }}
            >
                <div className="w-full h-fit flex flex-col justify-center items-center pt-4">
                    {props.banner ?? null}
                </div>
                <SidebarMobileDrawerSection
                    label="Topics"
                    icon={<LucideFolderPen className="w-3 h-3" />}
                    items={props.topics.map((t) => {
                        return {
                            label: t,
                            href: topicToUrl(t, BLOGURL),
                        };
                    })}
                />
                <SidebarMobileDrawerSection
                    label="Subjects"
                    icon={<LucidePaperclip className="w-3 h-3" />}
                    items={props.subjects.map((t) => {
                        return {
                            label: t,
                            href: subjectToUrl(t, BLOGURL),
                        };
                    })}
                />
            </motion.div>
        </>
    );
};

SidebarMobileDrawer.displayName = "SidebarMobileDrawer";
