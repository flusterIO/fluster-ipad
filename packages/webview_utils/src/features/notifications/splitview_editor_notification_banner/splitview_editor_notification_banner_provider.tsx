
"use client"
import React, { createContext, useReducer, useContext, ReactNode } from "react";
import { v4 } from "uuid"
import { SplitviewEditorNotificationBanner } from "./splitview_editor_notification_banner";
import { EditorNotificationBanner } from "./types";
import { useEventListener } from "@/state/hooks/use_event_listener";


export interface SplitviewEditorNotificationState {
    banners: EditorNotificationBanner[]
}

const defaultInitialValues: SplitviewEditorNotificationState = {
    banners: []
}

export const SplitviewEditorNotificationContext = createContext<SplitviewEditorNotificationState>(defaultInitialValues);

type SplitviewEditorNotificationContextActions = {
    type: "appendEditorNotificationBanner",
    payload: Omit<EditorNotificationBanner, "id">
} | {
    type: "removeEditorNotifcationBannerById",
    payload: string
}

export const SplitviewEditorNotificationDispatchContext = createContext<React.Dispatch<SplitviewEditorNotificationContextActions>>(null!);


export const useSplitviewEditorNotificationContext = () => useContext(SplitviewEditorNotificationContext)
export const useSplitviewEditorNotificationDispatch = () => useContext(SplitviewEditorNotificationDispatchContext)


export const SplitviewEditorNotificationContextReducer = (state: SplitviewEditorNotificationState, action: SplitviewEditorNotificationContextActions): SplitviewEditorNotificationState => {
    switch (action.type) {
        case "appendEditorNotificationBanner": {
            return {
                ...state,
                banners: [...state.banners, {
                    ...action.payload,
                    id: v4()
                }]
            }
        }
        case "removeEditorNotifcationBannerById": {
            return {
                ...state,
                banners: state.banners.filter((f) => f.id !== action.payload)
            }
        }

        default: {
            return state
        }
    }
}

SplitviewEditorNotificationContextReducer.displayName = "SplitviewEditorNotificationContextReducer"

export const SplitviewEditorNotificationHandler = (): ReactNode => {
    const [state, dispatch] = useReducer(
        SplitviewEditorNotificationContextReducer,
        defaultInitialValues,
    );

    useEventListener("show-editor-banner", (e) => {
        dispatch({
            type: "appendEditorNotificationBanner",
            payload: e.detail
        })
    })

    return (
        <SplitviewEditorNotificationContext.Provider value={state} >
            <SplitviewEditorNotificationDispatchContext.Provider value={dispatch}>
                <SplitviewEditorNotificationBanner />
            </SplitviewEditorNotificationDispatchContext.Provider>
        </SplitviewEditorNotificationContext.Provider>
    )
}

