import { createContext } from "react";
import { HealthState } from "./health_types";

export const HealthContext = createContext<HealthState>({
    report: null,
    have_checked_health: false,
});
