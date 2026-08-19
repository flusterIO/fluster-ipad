"use client";
import { type ReactNode, createContext, useReducer, useContext } from "react";
import { type DatabaseTableManager } from "../table_managers/table_manager";
import { type VisibilityState, type RowData } from "@tanstack/react-table";
import { UserWorkspaceTableManager } from "../table_managers/user_workspace_table_manager";
import { DatabaseTable } from "@conundrum/ts/codegen-typeshare";

export interface DatabaseTableState<DataType extends RowData> {
    /**
     * **TODO** Convert this to a DatabaseTable once that code has been generated in typescript
     */
    selectedTable: DatabaseTable | null;
    loading: boolean;
    tableManager: DatabaseTableManager<DataType> | null;
    visibility?: VisibilityState | null;
}

const defaultInitialValues: DatabaseTableState<RowData> = {
    selectedTable: null,
    loading: true,
    tableManager: null,
    visibility: null,
};

export const DatabaseTableContext =
    createContext<DatabaseTableState<RowData>>(defaultInitialValues);

type DatabaseTableContextActions =
    | { type: "set-selected-table"; payload: DatabaseTable | null }
    | {
        type: "set-loading";
        payload: boolean;
    }
    | {
        type: "set-visibility";
        payload: VisibilityState;
    };

export const DatabaseTableDispatchContext = createContext<
    React.Dispatch<DatabaseTableContextActions>
>(null!);

export const useDatabaseTableContext = <TData extends RowData>() =>
    useContext(DatabaseTableContext) as DatabaseTableState<TData>;
export const useDatabaseTableDispatch = () =>
    useContext(DatabaseTableDispatchContext);

const getSelectedTableManager = <TData extends RowData>(
    databaseTable: DatabaseTable,
): DatabaseTableManager<TData> => {
    switch (databaseTable) {
        case DatabaseTable.UserWorkspace: {
            return new UserWorkspaceTableManager() as DatabaseTableManager<TData>;
        }
    }
};

export const DatabaseTableContextReducer = <TData extends RowData>(
    state: DatabaseTableState<TData>,
    action: DatabaseTableContextActions,
): DatabaseTableState<TData> => {
    switch (action.type) {
        case "set-selected-table": {
            return {
                ...state,
                selectedTable: action.payload,
                tableManager: action.payload
                    ? getSelectedTableManager(action.payload)
                    : null,
            };
        }
        case "set-loading": {
            return {
                ...state,
                loading: action.payload,
            };
        }
        case "set-visibility": {
            return {
                ...state,
                visibility: action.payload,
            };
        }
        default: {
            return state;
        }
    }
};

DatabaseTableContextReducer.displayName = "DatabaseTableContextReducer";

interface DatabaseTableProviderProps<TData extends RowData> {
    children: ReactNode;
    initialValues?: Partial<DatabaseTableState<TData>>;
}

export const DatabaseTableProvider = <TData extends RowData>({
    children,
    initialValues,
}: DatabaseTableProviderProps<TData>) => {
    const [state, dispatch] = useReducer(
        DatabaseTableContextReducer,
        initialValues
            ? { ...initialValues, ...defaultInitialValues }
            : defaultInitialValues,
    );

    return (
        <DatabaseTableContext.Provider value={state}>
            <DatabaseTableDispatchContext.Provider value={dispatch}>
                {children}
            </DatabaseTableDispatchContext.Provider>
        </DatabaseTableContext.Provider>
    );
};
