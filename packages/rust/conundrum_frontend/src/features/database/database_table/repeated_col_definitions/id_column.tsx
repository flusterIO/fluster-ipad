import React from "react";
import { type ColumnDef } from "@tanstack/react-table";
import { DropdownHeaderCell } from "../cells/dropdown_header_cell";

interface IdRecord {
    id: string;
}

export const getIdColumnDefinition = <T extends IdRecord>(
    columnId = "id",
): ColumnDef<T> => {
    return {
        id: columnId,
        enableHiding: true,
        enableSorting: true,
        cell(props) {
            return props.getValue();
        },
        header(props) {
            return <DropdownHeaderCell content="Id" column={props.column} />;
        },
    };
};
