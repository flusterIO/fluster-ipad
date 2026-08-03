import { DatabasePanelKey } from "../database_panel_key";
import { type DatabaseState } from "./database_state";

export const databaseInitialState: DatabaseState = {
    selected_panel_key: DatabasePanelKey.UserWorkspace,
};
