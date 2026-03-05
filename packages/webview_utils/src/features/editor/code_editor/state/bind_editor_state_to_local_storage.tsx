import React, { type ReactNode } from 'react'
import { useCodeEditorContext, useCodeEditorDispatch } from './code_editor_provider'
import { useIsomorphicLayoutEffect } from '@/state/hooks/use_isomorphic_layout_effect'
import { useIsMounted } from '@/state/hooks/use_is_mounted'
import { EditorState, SplitviewEditorWebviewLocalStorageKeys } from '@/code_gen/typeshare/fluster_core_utilities'
import json from 'superjson'
import deepEqual from "fast-deep-equal"


export const BindEditorStateToLocalStorage = (): ReactNode => {
    const state = useCodeEditorContext()
    const dispatch = useCodeEditorDispatch()
    const isMounted = useIsMounted()
    useIsomorphicLayoutEffect(() => {
        const localStorageState = window.localStorage.getItem(SplitviewEditorWebviewLocalStorageKeys.EditorState)
        if (localStorageState) {
            const parsedStoredState = json.parse<EditorState>(localStorageState)
            if (state) {
                const stateIsMatch = deepEqual(state, parsedStoredState)
                if (stateIsMatch) {
                    return
                } else {
                    if (!isMounted) { // Only run this on the initial launch, otherwise prefer state presented to user.
                        return dispatch({
                            type: "loadSavedState",
                            payload: parsedStoredState
                        })
                    }
                }
            }
        }
        window.localStorage.setItem(SplitviewEditorWebviewLocalStorageKeys.EditorState, json.stringify(state))
    }, [state])
    return null
}


BindEditorStateToLocalStorage.displayName = "BindEditorStateToLocalStorage"
