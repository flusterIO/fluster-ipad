import React, {
    useEffect,
    useEffectEvent,
    useMemo,
    useState,
    type ReactNode,
} from "react";
import { type DatabasePanelKey } from "../database_panel_key";
import { connect } from "react-redux";
import { type AppState } from "@/state/initial_state";
import { DatabaseTable as DBTable } from "@conundrum/ts/codegen-typeshare";
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
} from "@tanstack/react-table";
import {
    useDatabaseTableContext,
    useDatabaseTableDispatch,
} from "./database_table_context/database_table_context";
import { fuzzyFilter } from "./table_utils/fuzzy_filter";
import { logMaybeObject } from "#/error_handling/utils/log_maybe_object";
import { DatabaseTablePagination } from "./database_table_pagination";
import { type DatabaseTableManager } from "./table_managers/table_manager";
import { DatabaseTableCombobox } from "#/settings/inputs/combobox/database_table_combobox";
import { useSearchParams } from "react-router";

const connector = connect((state: AppState) => ({
    panel_key: state.database.selected_panel_key,
}));

export const DatabaseTable = connector(<TData extends RowData>(): ReactNode => {
    const [sorting, setSorting] = useState<SortingState>([]);
    const { selectedTable, tableManager, visibility } =
        useDatabaseTableContext<TData>();
    const tableDispatch = useDatabaseTableDispatch();
    const [data, setData] = useState<TData[]>([]);
    const [sp, setSp] = useSearchParams();
    const searchParamTable = sp.get("db_table");

    const gatherData = useEffectEvent(async (tm: DatabaseTableManager<TData>) => {
        if (tableManager) {
            try {
                tableDispatch({
                    type: "set-loading",
                    payload: true,
                });
                const res = await tm.getData();
                setData(res);
                tableDispatch({
                    type: "set-loading",
                    payload: false,
                });
            } catch (err: unknown) {
                logMaybeObject("Error: ", err);
                tableDispatch({
                    type: "set-loading",
                    payload: false,
                });
            }
        }
    });

    useEffect(() => {
        if (tableManager) {
            gatherData(tableManager).catch((err: unknown) => {
                logMaybeObject("Error: ", err);
            });
        }
    }, [tableManager]);

    useEffect(() => {
        // eslint-disable-next-line @typescript-eslint/no-unsafe-enum-comparison
        if (searchParamTable && searchParamTable !== selectedTable) {
            tableDispatch({
                type: "set-selected-table",
                payload: searchParamTable as DBTable,
            });
        }
    }, [searchParamTable]);

    useEffect(() => {
        if (selectedTable) {
            sp.set("db_table", selectedTable);
            setSp(sp);
        }
    }, [selectedTable]);

    const [pagination, setPagination] = useState<PaginationState>({
        pageIndex: 0,
        pageSize: 10,
    });
    const columns = useMemo(() => {
        return tableManager?.getColumns();
    }, [tableManager]);

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
                payload:
                    typeof newVisiblity === "function"
                        ? newVisiblity(visibility ?? {})
                        : newVisiblity,
            });
        },
        state: {
            columnVisibility: visibility ?? undefined,
            sorting,
            globalFilter,
            pagination,
        },
    });
    return (
        <div className="@container/tableContainer flex flex-col min-h-[calc(100vh-4rem)]">
            <div className="w-full flex flex-row justify-center items-center @[640px]/tableContainer:justify-end">
                <DatabaseTableCombobox
                    value={selectedTable ?? DBTable.Cdrm}
                    className="w-full @[640px]/tableContainer:w-[min(450px,90%)]"
                    onValueChange={(val) => {
                        tableDispatch({
                            type: "set-selected-table",
                            payload: val,
                        });
                    }}
                    inputProps={{
                        className: "w-full",
                        placeholder: "Search Tables...",
                    }}
                />
            </div>
            <div className="grow flex flex-col justify-center items-center">
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
                                            {flexRender(
                                                cell.column.columnDef.cell,
                                                cell.getContext(),
                                            )}
                                        </TableCell>
                                    ))}
                                </TableRow>
                            ))
                        ) : (
                            <TableRow>
                                <TableCell
                                    colSpan={columns?.length ?? 1}
                                    className="h-24 text-center"
                                >
                                    {tableManager
                                        ? `No ${tableManager.entityName()} entities found`
                                        : "No entities found"}
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
                <DatabaseTablePagination table={table} />
            </div>
        </div>
    );
});

DatabaseTable.displayName = "DatabaseTable";
