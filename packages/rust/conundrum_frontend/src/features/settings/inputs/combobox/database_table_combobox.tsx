import React, { type ReactNode } from "react";

import { type DatabaseTable } from "@conundrum/ts/codegen-typeshare";
import { useTableDescriptions } from "#/database/state/hooks/use_table_descriptions";
import { type GeneralComboboxProps } from "./general_combobox";
import { GeneralLabeledCombobox } from "./general_labeled_combobox";

interface DatabaseTableSelectInputProps extends Omit<
    GeneralComboboxProps,
    "emptyText" | "options"
> {
    value: DatabaseTable;
    onValueChange: (val: DatabaseTable) => void;
    className?: string;
}

export const DatabaseTableCombobox = ({
    value,
    onValueChange,
    ...props
}: DatabaseTableSelectInputProps): ReactNode => {
    const data = useTableDescriptions();
    return (
        <GeneralLabeledCombobox
            {...props}
            emptyText="No tables found"
            label="Table"
            comboboxProps={{
                ...props.comboboxProps,
                value: data.data
                    ? (data.data.find((f) => {
                        // eslint-disable-next-line @typescript-eslint/no-unsafe-enum-comparison
                        return f.table === value;
                    })?.entity_name ?? "")
                    : "",
                onValueChange(value) {
                    console.log("value: ", value);
                    onValueChange(value as DatabaseTable);
                },
            }}
            options={
                data.data
                    ? data.data.map((item) => {
                        return {
                            value: item.table,
                            label: item.entity_name,
                            id: item.table,
                        };
                    })
                    : []
            }
        />
    );
};

DatabaseTableCombobox.displayName = "DatabaseTableSelectInput";
