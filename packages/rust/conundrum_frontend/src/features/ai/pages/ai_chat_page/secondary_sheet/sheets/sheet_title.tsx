import React, { useState, type ReactNode } from "react";
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
import { Button } from "@/components/shad/button";

const options: Record<SecondaryPanelKey, string> = {
    [SecondaryPanelKey.ChatSelect]: "Chat Select",
    [SecondaryPanelKey.AgentSelect]: "Agent Select",
    [SecondaryPanelKey.ToolExecHistory]: "Tool Calls",
    [SecondaryPanelKey.Logs]: "Logs",
};

export const SheetTitle = (): ReactNode => {
    const value = useSelector((state: AppState) => {
        return state.navigation.side_panel.active_panel;
    });
    const [open, setOpen] = useState(false);
    const dispatch = useDispatch();
    return (
        <div className="w-full h-fit my-4 px-4">
            <DropdownMenu
                open={open}
                onOpenChange={(newOpen) => {
                    if (!newOpen) {
                        setOpen(false);
                    }
                }}
            >
                <DropdownMenuTrigger
                    className="*:text-foreground"
                    render={(props) => {
                        return (
                            <Button
                                {...props}
                                variant={"outline"}
                                onClick={(e) => {
                                    e.stopPropagation();
                                    setOpen(!open);
                                }}
                                className="text-foreground"
                            >
                                {options[value]}
                            </Button>
                        );
                    }}
                />
                <DropdownMenuPortal>
                    <DropdownMenuContent>
                        {Object.values(options).map((k) => {
                            return (
                                <DropdownMenuItem
                                    key={k}
                                    onClick={(e) => {
                                        e.stopPropagation();
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
        </div>
    );
};

SheetTitle.displayName = "SheetTitle";
