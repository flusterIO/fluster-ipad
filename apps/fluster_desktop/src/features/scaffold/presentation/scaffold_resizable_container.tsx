import React, { useEffect, useMemo, useState, type ReactNode } from "react";
import { AppState } from "@/state/initial_state";
import { connect } from "react-redux";
import { motion } from "framer-motion";
import { PanelLeftContainer } from "#/panel_left/presentation/panel_left_container";
import { PanelRightContainer } from "#/panel_right/presentation/panel_right_container";
import { cn } from "@fluster/webview_utils";

const connector = connect((state: AppState) => ({
    panelLeftWidth: state.panelLeft.width,
    panelLeftOpen: state.panelLeft.open,
    panelRightWidth: state.panelRight.width,
    panelRightOpen: state.panelRight.open,
}));

interface ScaffoldResizableContainerProps {
    panelLeftWidth: number;
    panelRightWidth: number;
    panelLeftOpen: boolean;
    panelRightOpen: boolean;
    children: ReactNode;
}

/// All values are in pixels, not percentages or ratios.
interface RatioData {
    left: number;
    right: number;
}

const Divider = ({
    setPortion,
}: {
    setPortion: (n: number) => void;
}): ReactNode => {
    return (
        <div className="w-full h-full flex-col flex justify-center items-center">
            <motion.div
                className="w-2 h-16 rounded-lg bg-muted cursor-pointer"
                drag="x"
                onDrag={(e: DragEvent) => {
                    setPortion(e.clientX / window.innerWidth);
                }}
                dragMomentum={false}
                dragElastic={0}
                whileDrag={{
                    scale: 1.1,
                }}
                dragConstraints={{
                    left: 0,
                    right: 0,
                }}
            />
        </div>
    );
};

export const ScaffoldResizableContainer = connector(
    ({
        panelLeftWidth,
        panelRightWidth,
        panelLeftOpen,
        panelRightOpen,
        children,
    }: ScaffoldResizableContainerProps): ReactNode => {
        const MIN_WIDTH = 100;
        const maxWidth = window.innerWidth; // This willl need to change once sidebar navigation is added.
        const [portions, setPortions] = useState<RatioData>({
            left: Math.max(maxWidth * panelLeftWidth, MIN_WIDTH),
            right: Math.max(maxWidth * panelRightWidth, MIN_WIDTH),
        });

        const gridTemplateColumns = useMemo(() => {
            if (!panelLeftOpen && !panelRightOpen) {
                return "1fr";
            }
            if (panelLeftOpen && panelRightOpen) {
                return `${panelLeftOpen ? portions.left : 0}px 2px 1fr 2px ${panelRightOpen ? portions.right : 0}px`;
            }
            if (panelLeftOpen) {
                return `${panelLeftOpen ? portions.left : 0}px 2px 1fr`;
            }
            if (panelRightOpen) {
                return `1fr 2px ${panelRightOpen ? portions.right : 0}px`;
            }
        }, [panelLeftOpen, panelRightOpen, portions]);

        const handleWindowResize = (): void => {
            setPortions({
                left: Math.max(maxWidth * panelLeftWidth, MIN_WIDTH),
                right: Math.max(maxWidth * panelRightWidth, MIN_WIDTH),
            });
        };

        useEffect(() => {
            window.addEventListener("resize", handleWindowResize);
            return () => window.removeEventListener("resize", handleWindowResize);
        }, []);

        console.log("gridTemplateColumns: ", gridTemplateColumns);
        return (
            <div
                className="w-full h-[calc(100vh-2rem)] grid"
                style={{
                    gridTemplateColumns,
                }}
            >
                {panelLeftOpen ? (
                    <>
                        <div
                            className={
                                "px-4 py-6 bg-muted/30 pt-8 h-full rounded-tr-lg rounded-br-lg"
                            }
                        >
                            <PanelLeftContainer />
                        </div>
                        <Divider
                            setPortion={(e) => {
                                setPortions({
                                    ...portions,
                                    left: Math.max(e * window.innerWidth, MIN_WIDTH),
                                });
                            }}
                        />
                    </>
                ) : null}
                <div className="scroll-target h-full overflow-y-auto min-scrollbar">
                    {children}
                </div>
                {panelRightOpen ? (
                    <>
                        <Divider
                            setPortion={(e) => {
                                setPortions({
                                    ...portions,
                                    right: Math.max(window.innerWidth * (1 - e), MIN_WIDTH),
                                });
                            }}
                        />

                        <div
                            className={cn(
                                "px-4 py-6 bg-muted/30 pt-8 h-full rounded-tl-lg rounded-bl-lg",
                            )}
                        >
                            <PanelRightContainer />
                        </div>
                    </>
                ) : null}
            </div>
        );
    },
);

ScaffoldResizableContainer.displayName = "ScaffoldResizableContainer";
