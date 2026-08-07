import React, { useEffectEvent, useMemo, useState, type ReactNode } from "react";
import { type DatabasePanelKey } from "../database_panel_key";

import { connect } from "react-redux";
import { type AppState } from "@/state/initial_state";
import {
    TableHeader,
    TableRow,
    TableHead,
    TableBody,
    TableCell,
    Table,
} from "@/components/shad/table";
import {
    flexRender,
    getCoreRowModel,
    getFilteredRowModel,
    getPaginationRowModel,
    getSortedRowModel,
    type PaginationState,
    type RowData,
    type SortingState,
    useReactTable,
    type VisibilityState,
} from "@tanstack/react-table";
import { useDatabaseTableContext, useDatabaseTableDispatch } from "./database_table_context/database_table_context";
import { fuzzyFilter } from "./table_utils/fuzzy_filter";
import { logMaybeObject } from "#/error_handling/utils/log_maybe_object";
import { DatabaseTablePagination } from "./database_table_pagination";

const connector = connect((state: AppState) => ({
    panel_key: state.database.selected_panel_key,
}));

interface DatabaseTableProps {
    panel_key: DatabasePanelKey;
}

export const DatabaseTable = connector(
    <TData extends RowData>({ panel_key }: DatabaseTableProps): ReactNode => {
        const [sorting, setSorting] = useState<SortingState>([]);
        const { selectedTable, tableManager, visibility, loading } = useDatabaseTableContext<TData>()
        const tableDispatch = useDatabaseTableDispatch();
        const [data, setData] = useState<TData[]>([])

        useEffectEvent(async () => {
            if (tableManager) {
                try {
                    tableDispatch({
                        type: "set-loading",
                        payload: true
                    })
                    const res = await tableManager.getData();
                    setData(res)
                    tableDispatch({
                        type: "set-loading",
                        payload: false
                    })
                } catch (err: unknown) {
                    logMaybeObject("Error: ", err)
                    tableDispatch({
                        type: "set-loading",
                        payload: false
                    })
                }

            }
        })

        const [pagination, setPagination] = useState<PaginationState>({
            pageIndex: 0,
            pageSize: 10,
        });
        const columns = useMemo(() => {
            return tableManager?.getColumns()
        }, [tableManager])

        const [globalFilter, setGlobalFilter] = useState<string>("");
        const table = useReactTable({
            autoResetPageIndex: true,
            columns: columns ?? [],
            data,
            manualPagination: false,
            getCoreRowModel: getCoreRowModel(),
            rowCount: data.length,
            onSortingChange: setSorting,
            getSortedRowModel: getSortedRowModel(),
            getPaginationRowModel: getPaginationRowModel(),
            getFilteredRowModel: getFilteredRowModel(),
            onPaginationChange: setPagination,
            globalFilterFn: fuzzyFilter,
            onGlobalFilterChange: setGlobalFilter,
            onColumnVisibilityChange: (newVisiblity) => {
                tableDispatch({
                    type: "set-visibility",
                    payload: typeof newVisiblity === "function" ? newVisiblity(visibility ?? {}) : newVisiblity
                })
            },
            state: {
                columnVisibility: visibility ?? undefined,
                sorting,
                globalFilter,
                pagination,
            },
        });
        return (
            <>
                <Table>
                    <TableHeader>
                        {table.getHeaderGroups().map((headerGroup) => (
                            <TableRow key={headerGroup.id}>
                                {headerGroup.headers.map((header) => {
                                    return (
                                        <TableHead key={header.id}>
                                            {header.isPlaceholder
                                                ? null
                                                : flexRender(
                                                    header.column.columnDef.header,
                                                    header.getContext(),
                                                )}
                                        </TableHead>
                                    );
                                })}
                            </TableRow>
                        ))}
                    </TableHeader>
                    <TableBody>
                        {table.getPaginationRowModel().rows?.length ? (
                            table.getPaginationRowModel().rows.map((row) => (
                                <TableRow
                                    key={row.id}
                                    data-state={row.getIsSelected() && "selected"}
                                    className="cursor-pointer"
                                >
                                    {row.getVisibleCells().map((cell) => (
                                        <TableCell key={cell.id}>
                                            {flexRender(cell.column.columnDef.cell, cell.getContext())}
                                        </TableCell>
                                    ))}
                                </TableRow>
                            ))
                        ) : (
                            <TableRow>
                                <TableCell colSpan={columns?.length ?? 1} className="h-24 text-center">
                                    No results.
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
                <DatabaseTablePagination table={table} />
            </>
        );
    },
);

DatabaseTable.displayName = "DatabaseTable";
