import {
    DropdownMenu,
    DropdownMenuCheckboxItem,
    DropdownMenuContent,
    DropdownMenuPortal,
    DropdownMenuTrigger,
} from "@/components/shad/dropdown-menu";
import { Input } from "@/components/shad/input";
import React, { type ReactNode, type ComponentProps } from "react";

export type OptionRecord<K extends string> = Record<K, boolean>;

interface Item {
    checked: boolean;
    label: ReactNode;
    id: string;
}

interface GeneralCheckedDropdownInputProps {
    options: Item[];
    onCheckedChange: (item: Item) => void;
    className?: string;
    inputProps?: ComponentProps<typeof Input>;
    contentProps?: ComponentProps<typeof DropdownMenuContent>;
    itemProps?: Omit<
        ComponentProps<typeof DropdownMenuCheckboxItem>,
        "key" | "checked" | "children"
    >;
}

export const GeneralCheckedDropdownInput = ({
    options,
    className,
    contentProps,
    children,
    inputProps,
    itemProps,
}: GeneralCheckedDropdownInputProps): ReactNode => {
    return (
        <div className={className}>
            <div className={""}>
                <Input {...inputProps} />
            </div>
            <DropdownMenuPortal>
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>{children}</DropdownMenuTrigger>
                    <DropdownMenuContent {...contentProps}>
                        {options.map((item) => {
                            return (
                                <DropdownMenuCheckboxItem
                                    {...itemProps}
                                    key={item.id}
                                    checked={item.checked}
                                >
                                    {item.label}
                                </DropdownMenuCheckboxItem>
                            );
                        })}
                    </DropdownMenuContent>
                </DropdownMenu>
            </DropdownMenuPortal>
        </div>
    );
};

GeneralCheckedDropdownInput.displayName = "GeneralCheckedDropdownInput";
