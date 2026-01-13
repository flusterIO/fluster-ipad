import React, { type ReactNode } from "react";
import { CommandPaletteItem } from "../data/command_palette_item";
import { ChevronRight } from "lucide-react";
import { cn } from "@fluster/webview_utils";

interface CommandPaletteItemProps {
    item: CommandPaletteItem;
    idx: number;
    focusedIndex: number;
}

export const CommandPaletteItemComponent = ({
    item,
    focusedIndex,
    idx,
}: CommandPaletteItemProps): ReactNode => {
    const Icon = item.icon;
    return (
        <div
            className={cn(
                "grid w-full p-2 gap-x-2 place-items-center border-l-[2px] border-b-[1px] border-b-foreground/10",
                focusedIndex === idx ? "border-l-primary" : "border-l-transparent",
            )}
            style={{
                gridTemplateColumns: item.children ? "1.5rem 1fr 2rem" : "1.5rem 1fr",
            }}
        >
            <Icon className="text-muted-foreground w-4 h-4" />
            <div
                className="w-full text-sm"
                dangerouslySetInnerHTML={{ __html: item.label }}
            />
            {item.children ? <ChevronRight /> : null}
        </div>
    );
};

CommandPaletteItemComponent.displayName = "CommandPaletteItemComponent";
