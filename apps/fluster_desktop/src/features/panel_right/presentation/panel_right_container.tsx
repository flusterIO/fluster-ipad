import React, { type ReactNode } from "react";

import { AppState } from "@/state/initial_state";
import { connect, useDispatch, useSelector } from "react-redux";
import {
    PanelRightItemId,
    panelRightItems,
    PanelRightState,
} from "../state/panel_right_state";
import { QuickSettingsSidePanel } from "./panels/quick_settings/quick_settings_side_panel";
import { PanelSelect } from "#/panel_left/presentation/panel_select";
import { setPanelRightPanel } from "../state/panel_right_slice";
import { setPanelLeftPanel } from "#/panel_left/state/panel_left_slice";
import { panelLeftItems } from "#/panel_left/state/panel_left_state";

const connector = connect((state: AppState) => ({
    panelRight: state.panelRight.selectedPanel,
}));

interface PanelRightContainerProps {
    panelRight: PanelRightState["selectedPanel"];
}

const PanelRightSwitch = connector(
    ({ panelRight }: PanelRightContainerProps) => {
        switch (panelRight.id) {
            case PanelRightItemId.quickSettings:
                return <QuickSettingsSidePanel />;
        }
    },
);

export const PanelRightContainer = (): ReactNode => {
    const dispatch = useDispatch();
    const panelLeft = useSelector(
        (appState: AppState) => appState.panelLeft.selectedPanel,
    );
    const panelRight = useSelector(
        (appState: AppState) => appState.panelRight.selectedPanel,
    );
    return (
        <div className="h-full max-h-[calc(100vh-2rem)]">
            <PanelSelect
                onChange={(item) => dispatch(setPanelRightPanel(item))}
                items={panelRightItems}
                value={panelRight}
            />
            <PanelRightSwitch />
        </div>
    );
};

PanelRightContainer.displayName = "PanelRightContainer";
