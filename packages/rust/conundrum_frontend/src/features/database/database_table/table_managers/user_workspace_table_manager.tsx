import React from "react";
import { type WorkspaceByPredicate } from "#/database/db_utility_types/workspace";
import { type ColumnDef } from "@tanstack/react-table";
import { DatabaseTableManager } from "./table_manager";
import { client } from "@/app/rspc_client";
import { logMaybeObject } from "#/error_handling/utils/log_maybe_object";
import { getIdColumnDefinition } from "../repeated_col_definitions/id_column";
import { FilePathCell } from "../cells/file_path_cell";
import { DropdownHeaderCell, headerCell } from "../cells/dropdown_header_cell";
import { Checkbox } from "@/components/shad/checkbox";

export class UserWorkspaceTableManager extends DatabaseTableManager<WorkspaceByPredicate> {
    constructor() {
        super({
            table: "user_workspace",
        });
    }
    async getData(
        perPage: number,
        page: number,
    ): Promise<WorkspaceByPredicate[]> {
        try {
            const res = await client.query([
                "crud.user_workspace.get_by_predicate",
                {
                    predicate: null,
                    pagination: {
                        page,
                        per_page: perPage,
                    },
                },
            ]);
            console.log("res: ", res);
            return res;
        } catch (err: unknown) {
            logMaybeObject("Error: ", err);
            return [];
        }
    }
    entityName(): string {
        return "Workspace";
    }
    getColumns(): ColumnDef<WorkspaceByPredicate>[] {
        return [
            {
                id: "label",
                accessorKey: "label",
                header: headerCell("Label"),
            },
            {
                id: "root",
                accessorKey: "root",
                header: headerCell("Path"),
                cell: (props) => {
                    const value = props.getValue();
                    return (
                        <FilePathCell>
                            {typeof value === "string" ? value : ""}
                        </FilePathCell>
                    );
                },
            },
            {
                id: "respect_gitignore",
                accessorKey: "respect_gitignore",
                header: "Respect .gitignore",
                cell: (props) => {
                    const r = props.getValue() as boolean;
                    return <Checkbox disabled checked={r} />;
                },
            },
            {
                id: "ignore_hidden",
                accessorKey: "ignore_hidden",
                header: "Ignore Hidden",
                cell: (props) => {
                    const r = props.getValue() as boolean;
                    return <Checkbox disabled checked={r} />;
                },
            },
        ];
    }
}
