import { GeneralCheckedDropdownInput } from "#/settings/inputs/dropdown/general_checked_dropdown";
import { type DatabaseTable } from "@/codegen/bindings";
import React, { type ReactNode } from "react";

export const DatabaseTableColumnVisibility = (
    { visibility = {}, labelMap }: { visibility: Partial<Record<DatabaseTable, boolean>>, labelMap: Partial<Record<DatabaseTable, string>> }
): ReactNode => {
    return (
        <div className="grow w-full @[640px]/tableContainer:w-auto">
            <GeneralCheckedDropdownInput
                options={Object.keys(visibility).map((k) => {
                    return {
                        label: (k in labelMap ? labelMap[k] : k) as string,
                        id: k,
                        checked: visibility[k] ?? false
                    }
                })}
            />
        </div>
    );
};

DatabaseTableColumnVisibility.displayName = "DatabaseTableColumnVisibility";
