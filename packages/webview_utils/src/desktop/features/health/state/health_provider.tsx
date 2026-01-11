import React, { ReactNode, useEffect, useReducer } from "react";
import { connect } from "react-redux";
import { HealthDispatchContext } from "./health_dispatch_context";
import { useNavigate } from "react-router";
import { HealthContext } from "./health_context";
import { HealthContextReducer } from "./health_slice";
import { HealthState } from "./health_types";
import { AppState } from "src/desktop/core/state/initial_state";
import { AppRoutes } from "../../navigation/data/app_routes";
import { useEventListener } from "@/state/hooks/use_event_listener";
import { useIsomorphicLayoutEffect } from "@/state/hooks/use_isomorphic_layout_effect";
import { commands } from "@fluster/desktop_bindings";

const connector = connect((state: AppState) => ({
    notesDirectoryIsSet: Boolean(state.settings.notesDirectory.length),
}));

interface HealthProviderProps {
    children: ReactNode;
    initialValues?: Partial<HealthState>;
    notesDirectoryIsSet: boolean;
}

declare global {
    interface WindowEventMap {
        "request-new-health-report": CustomEvent<object>;
        "set-health-as-being-checked": CustomEvent<object>;
    }
}

const defaultInitialValues: HealthState = {
    report: null,
    have_checked_health: false,
};

export const HealthProvider = connector(
    ({ children, initialValues, notesDirectoryIsSet }: HealthProviderProps) => {
        const [state, dispatch] = useReducer(
            HealthContextReducer,
            initialValues
                ? { ...initialValues, ...defaultInitialValues }
                : defaultInitialValues
        );

        const checkHealth = async (): Promise<void> => {
            const res = await commands.getDesktopHealthReport();
            dispatch({
                type: "set_health_report",
                payload: {
                    ...res,
                    hasSetNotesDir: notesDirectoryIsSet,
                },
            });
        };

        useEventListener("request-new-health-report", () => {
            checkHealth();
        });

        useEventListener("set-health-as-being-checked", () => {
            dispatch({
                type: "set_health_report_as_being_checked",
                payload: null,
            });
        });

        const nav = useNavigate();

        /// FIXME: Disabled automatically checking health for now because it was causing issues with navigation on the onboarding screens. Fix this later if it becomes too cumbersome to do it through events.
        useEffect(() => {
            if (!state.report?.healthy && state.have_checked_health) {
                nav(AppRoutes.onboarding);
            }
            /* eslint-disable-next-line  --  */
        }, [state.have_checked_health]);

        useIsomorphicLayoutEffect(() => {
            if (state.have_checked_health === false) {
                checkHealth();
            }
        }, [state.have_checked_health]);

        return (
            <HealthContext.Provider value={state}>
                <HealthDispatchContext.Provider value={dispatch}>
                    {children}
                </HealthDispatchContext.Provider>
            </HealthContext.Provider>
        );
    }
);
