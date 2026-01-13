import { Dispatch } from "@reduxjs/toolkit";
import { FC } from "react";

export class CommandPaletteItem {
    label: string;
    icon: FC<{ className?: string }>;
    children: CommandPaletteItem[];
    action?: (d: Dispatch) => void;
    constructor(props: {
        label: string;
        icon: FC<{ className?: string }>;
        children?: CommandPaletteItem[];
        action?: (d: Dispatch) => void;
    }) {
        this.label = props.label;
        this.icon = props.icon;
        this.children = props.children ?? [];
        this.action = props.action;
    }
}
