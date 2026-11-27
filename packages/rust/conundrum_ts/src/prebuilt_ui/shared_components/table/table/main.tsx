import { type RowData } from "@tanstack/react-table";
import React, { type ReactNode } from "react";
import { type getTableColumns } from "../methods/get_table_columns";

interface GenericTableProps<TData extends RowData> {
    columns: ReturnType<typeof getTableColumns<TData>>;
}

export const GenericTable = <TData extends RowData>({
    columns,
}: GenericTableProps<TData>): ReactNode => {
    return <div></div>;
};

GenericTable.displayName = "GenericTable";
