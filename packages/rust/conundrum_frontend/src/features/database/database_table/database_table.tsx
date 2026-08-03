import React, { type ReactNode } from "react";
import { type DatabasePanelKey } from "../database_panel_key";

import { connect } from "react-redux";
import { type AppState } from "@/state/initial_state";

const connector = connect((state: AppState) => ({
    panel_key: state.database.selected_panel_key,
}));

interface DatabaseTableProps {
    panel_key: DatabasePanelKey;
}

export const DatabaseTable = connector(
    ({ panel_key }: DatabaseTableProps): ReactNode => {
        return <div>Database Table</div>;
    },
);

DatabaseTable.displayName = "DatabaseTable";
