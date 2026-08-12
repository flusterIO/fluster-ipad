import {
    SelectContent,
    SelectTrigger,
    SelectValue,
    SelectItem,
    Select,
} from "@/components/shad/select";
import { cn } from "@/utils/shad_utils";
import { Button } from "@/components/shad/button";
import {
    ChevronsLeft,
    ChevronLeft,
    ChevronRight,
    ChevronsRight,
} from "lucide-react";
import React from "react";
import type { Table } from "@tanstack/react-table";

export interface DatabaseTablePaginationProps<TData extends object> {
    hidePerPage?: boolean;
    hideSelectedCount?: boolean;
    table: Table<TData>;
    classes?: {
        container?: string;
    };
}

export const DatabaseTablePagination = <TData extends object>({
    table,
    hidePerPage,
    hideSelectedCount,
    classes = {},
}: DatabaseTablePaginationProps<TData>) => {
    return (
        <div
            className={cn(
                "w-full @container/table_footer flex items-center justify-between px-2",
                classes.container,
            )}
        >
            {!hideSelectedCount ? (
                <>
                    <div className="hidden text-muted-foreground @[768px]/table_footer:inline-block flex-1 text-sm">
                        {table.getFilteredSelectedRowModel().rows.length} of{" "}
                        {table.getFilteredRowModel().rows.length} row(s) selected.
                    </div>
                    <div className="inline-block @[768px]/table_footer:hidden" />
                </>
            ) : (
                <div />
            )}
            <div className="flex items-center space-x-6 lg:space-x-8">
                {!hidePerPage ? (
                    <div className="hidden items-center space-x-2 @[450px]/table_footer:flex">
                        <p className="text-sm font-medium">Rows per page</p>
                        <Select
                            value={`${table.getState().pagination.pageSize}`}
                            onValueChange={(value) => {
                                table.setPageSize(Number(value));
                            }}
                        >
                            <SelectTrigger className="h-8 w-17.5">
                                <SelectValue
                                    placeholder={table.getState().pagination.pageSize}
                                />
                            </SelectTrigger>
                            <SelectContent side="top">
                                {[10, 20, 25, 30, 40, 50, Number.MAX_SAFE_INTEGER].map(
                                    (pageSize) => (
                                        <SelectItem
                                            className="text-foreground"
                                            key={pageSize}
                                            value={`${pageSize}`}
                                        >
                                            {pageSize === Number.MAX_SAFE_INTEGER ? "All" : pageSize}
                                        </SelectItem>
                                    ),
                                )}
                            </SelectContent>
                        </Select>
                    </div>
                ) : (
                    <div />
                )}
                <div className="flex w-[100px] items-center justify-center text-sm font-medium">
                    Page {table.getState().pagination.pageIndex + 1} of{" "}
                    {table.getPageCount()}
                </div>
                <div className="flex items-center space-x-2">
                    <Button
                        variant="outline"
                        size="icon"
                        className="hidden size-8 lg:flex"
                        onClick={() => {
                            table.setPageIndex(0);
                        }}
                        disabled={!table.getCanPreviousPage()}
                    >
                        <span className="sr-only">Go to first page</span>
                        <ChevronsLeft />
                    </Button>
                    <Button
                        variant="outline"
                        size="icon"
                        className="size-8"
                        onClick={() => {
                            table.previousPage();
                        }}
                        disabled={!table.getCanPreviousPage()}
                    >
                        <span className="sr-only">Go to previous page</span>
                        <ChevronLeft />
                    </Button>
                    <Button
                        variant="outline"
                        size="icon"
                        className="size-8"
                        onClick={() => {
                            table.nextPage();
                        }}
                        disabled={!table.getCanNextPage()}
                    >
                        <span className="sr-only">Go to next page</span>
                        <ChevronRight />
                    </Button>
                    <Button
                        variant="outline"
                        size="icon"
                        className="hidden size-8 lg:flex"
                        onClick={() => {
                            table.setPageIndex(table.getPageCount() - 1);
                        }}
                        disabled={!table.getCanNextPage()}
                    >
                        <span className="sr-only">Go to last page</span>
                        <ChevronsRight />
                    </Button>
                </div>
            </div>
        </div>
    );
};
