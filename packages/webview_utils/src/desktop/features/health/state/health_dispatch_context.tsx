"use client";
import { createContext } from "react";
import { HealthContextActions } from "./health_types";

export const HealthDispatchContext = createContext<
    React.Dispatch<HealthContextActions>
>(null!);
