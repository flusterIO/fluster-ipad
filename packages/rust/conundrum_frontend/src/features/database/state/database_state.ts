import { type BackendStatus } from "../db_utility_types/health";

export interface DatabaseState {
    backend_status: BackendStatus | null;
}
