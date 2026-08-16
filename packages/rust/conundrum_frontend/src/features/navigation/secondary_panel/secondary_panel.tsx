import { AnimatePresence, motion } from "framer-motion";
import { type AppState } from "@/state/initial_state";
import React, { type ReactNode } from "react";
import { connect, useDispatch } from "react-redux";
import { setSidePanelOpen } from "../state/navigation_slice";
import { Button } from "@/components/shad/button";
import { PanelRightOpenIcon, XIcon } from "lucide-react";
import { type SecondaryPanelKey } from "./secondary_panel_key";
import { LogPanel } from "./panels/log_panel/log_panel";
const MotionButton = motion.create(Button);

const connector = connect((state: AppState) => ({
    open: state.navigation.side_panel.open,
    activePanel: state.navigation.side_panel.active_panel,
}));

interface SecondaryPanelProps {
    open: boolean;
    activePanel: SecondaryPanelKey;
}

export const SecondaryPanel = connector(
    ({ open }: SecondaryPanelProps): ReactNode => {
        const dispatch = useDispatch();

        return (
            <>
                <motion.div
                    key={"side-panel"}
                    className="origin-top-right w-[min(450px,80vw)] absolute right-0 bottom-0 h-screen bg-fd-card text-fd-card-foreground p-4 border-l"
                    variants={{
                        show: {
                            scale: 1,
                            opacity: 1,
                        },
                        hide: {
                            scale: 0,
                            opacity: 0,
                        },
                    }}
                    animate={open ? "show" : "hide"}
                    initial={open ? "show" : "hide"}
                >
                    <XIcon
                        className="w-4 h-4 cursor-pointer top-4 right-4 text-foreground absolute"
                        onClick={() => {
                            dispatch(setSidePanelOpen(false));
                        }}
                    />
                    <LogPanel />
                </motion.div>
            </>
        );
    },
);

SecondaryPanel.displayName = "SecondaryPanel";
