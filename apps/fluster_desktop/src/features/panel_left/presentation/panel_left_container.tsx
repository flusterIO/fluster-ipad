import { AppState } from "@/state/initial_state";
import React, { type ReactNode } from "react";
import { connect, useDispatch, useSelector } from "react-redux";
import {
    PanelLeftItemId,
    panelLeftItems,
    PanelLeftState,
} from "../state/panel_left_state";
import { setPanelLeftPanel } from "../state/panel_left_slice";
import { PanelSelect } from "./panel_select";
import { QuickNavigationSidePanel } from "./panels/quick_navigation/quick_navigation_side_panel";

const connector = connect((state: AppState) => ({
    panelLeft: state.panelLeft.selectedPanel,
}));

interface PanelLeftContainerProps {
    panelLeft: PanelLeftState["selectedPanel"];
}

const PanelLeftSwitch = connector(({ panelLeft }: PanelLeftContainerProps) => {
    switch (panelLeft.id) {
        case PanelLeftItemId.quickNavigation:
            return <QuickNavigationSidePanel />;
    }
});

export const PanelLeftContainer = connector(
    ({ panelLeft }: PanelLeftContainerProps): ReactNode => {
        const dispatch = useDispatch();
        return (
            <div className="h-[calc(100vh-2rem)] overflow-x-hidden">
                <PanelSelect
                    onChange={(item) => dispatch(setPanelLeftPanel(item))}
                    items={panelLeftItems}
                    value={panelLeft}
                />
                <PanelLeftSwitch />
            </div>
        );
    },
);

PanelLeftContainer.displayName = "PanelLeftContainer";
