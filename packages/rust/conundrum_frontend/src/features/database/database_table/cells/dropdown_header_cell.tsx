import { Button } from "@/components/shad/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from "@/components/shad/dropdown-menu";
import { type Column } from "@tanstack/react-table";
import { ArrowDown, ArrowUp, ChevronsUpDown, EyeOff } from "lucide-react";
import React, { type ReactNode } from "react";

interface DropdownHeaderCellProps<RowType> {
    content: ReactNode;
    column: Column<RowType>;
}

export const DropdownHeaderCell = <RowType extends object>({
    content,
    column,
}: DropdownHeaderCellProps<RowType>): ReactNode => {
    return (
        <div className="w-fit ml-2 max-w-[calc(100%-0.5rem)]">
            <DropdownMenu>
                <DropdownMenuTrigger
                    render={(props) => {
                        return (
                            <Button
                                {...props}
                                variant="ghost"
                                size="sm"
                                className="-ml-3 h-8 data-[state=open]:bg-accent"
                            >
                                <span>{content}</span>
                                {column.getIsSorted() === "desc" ? (
                                    <ArrowDown />
                                ) : column.getIsSorted() == "asc" ? (
                                    <ArrowUp />
                                ) : (
                                    <ChevronsUpDown />
                                )}
                            </Button>
                        );
                    }}
                />
                <DropdownMenuContent align="start">
                    <DropdownMenuItem
                        onClick={() => {
                            column.toggleSorting(false);
                        }}
                    >
                        <ArrowUp /> Asc
                    </DropdownMenuItem>
                    <DropdownMenuItem
                        onClick={() => {
                            column.toggleSorting(true);
                        }}
                    >
                        <ArrowDown /> Desc
                    </DropdownMenuItem>
                    <DropdownMenuItem
                        onClick={() => {
                            column.toggleVisibility(false);
                        }}
                    >
                        <EyeOff /> Hide
                    </DropdownMenuItem>
                </DropdownMenuContent>
            </DropdownMenu>
        </div>
    );
};

DropdownHeaderCell.displayName = "DropdownHeaderCell";

export const headerCell = <T extends object>(headerCellLabel: string) => {
    return (props: { column: Column<T> }) => {
        return (
            <DropdownHeaderCell content={headerCellLabel} column={props.column} />
        );
    };
};
