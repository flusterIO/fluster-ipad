import { type RowData, type ColumnDef } from "@tanstack/react-table";

export const getTableColumns = <
    TData extends RowData,
>(): ColumnDef<TData>[] => {
    return [];
};
