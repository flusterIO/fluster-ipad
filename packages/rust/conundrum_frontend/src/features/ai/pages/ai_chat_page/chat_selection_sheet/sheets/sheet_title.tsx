import { SheetHeader, SheetTitle as St } from "@/components/shad/sheet";
import React, { type ReactNode } from "react";
import { SecondaryPanelKey } from "#/navigation/secondary_panel/secondary_panel_key";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuPortal,
    DropdownMenuTrigger,
} from "@/components/shad/dropdown-menu";
import { useDispatch, useSelector } from "react-redux";
import { type AppState } from "@/state/initial_state";
import { setSecondaryActivePanel } from "#/navigation/state/navigation_slice";

const options: Record<SecondaryPanelKey, string> = {
    [SecondaryPanelKey.AgentSelect]: "Agent Select",
    [SecondaryPanelKey.Logs]: "Logs",
    [SecondaryPanelKey.ChatSelect]: "Chat Select",
    [SecondaryPanelKey.ToolExecHistory]: "Tool Calls",
};

export const SheetTitle = (): ReactNode => {
    const value = useSelector((state: AppState) => {
        return state.navigation.side_panel.active_panel;
    });
    const dispatch = useDispatch();
    return (
        <SheetHeader className="w-full">
            <St className="w-full">
                <DropdownMenu>
                    <DropdownMenuTrigger>{options[value]}</DropdownMenuTrigger>
                    <DropdownMenuPortal>
                        <DropdownMenuContent>
                            {Object.values(options).map((k) => {
                                return (
                                    <DropdownMenuItem
                                        key={k}
                                        onClick={() => {
                                            const value = Object.entries(options).find((f) => {
                                                return f[1] === k;
                                            })?.[0] as unknown as SecondaryPanelKey;
                                            dispatch(setSecondaryActivePanel(value));
                                        }}
                                    >
                                        {k}
                                    </DropdownMenuItem>
                                );
                            })}
                        </DropdownMenuContent>
                    </DropdownMenuPortal>
                </DropdownMenu>
            </St>
        </SheetHeader>
    );
};

SheetTitle.displayName = "SheetTitle";
