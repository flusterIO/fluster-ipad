import {
    Combobox,
    ComboboxInput,
    ComboboxContent,
    ComboboxEmpty,
    ComboboxItem,
    ComboboxList,
} from "@/components/shad/combobox";
import React, { type ComponentProps, type ReactNode } from "react";

export interface GeneralComboboxProps {
    options: { id: string; label: ReactNode; value: string }[];
    emptyText?: ReactNode;
    comboboxProps?: Omit<ComponentProps<typeof Combobox>, "items">;
    inputProps?: ComponentProps<typeof ComboboxInput>;
    contentProps?: ComponentProps<typeof ComboboxContent>;
    listProps?: ComponentProps<typeof ComboboxList>;
}

export const GeneralCombobox = ({
    options,
    emptyText,
    inputProps,
    contentProps,
    comboboxProps,
    listProps,
}: GeneralComboboxProps): ReactNode => {
    return (
        <Combobox {...comboboxProps} items={options}>
            <ComboboxInput {...inputProps} />
            <ComboboxContent {...contentProps}>
                <ComboboxEmpty>{emptyText ?? "No items found"}</ComboboxEmpty>
                <ComboboxList {...listProps}>
                    {(item: GeneralComboboxProps["options"][number]) => {
                        return (
                            <ComboboxItem
                                /* className="text-foreground" */
                                enterKeyHint="go"
                                key={item.id}
                                value={item.value}
                            >
                                {item.label}
                            </ComboboxItem>
                        );
                    }}
                </ComboboxList>
            </ComboboxContent>
        </Combobox>
    );
};

GeneralCombobox.displayName = "GeneralCombobox";
