import { useCodeEditorDispatch } from '#/editor/code_editor/state/code_editor_provider'
import { ResponsiveSplitViewEditor } from '#/split_view_editor/components/responsive_splitview_editor'
import { SplitviewEditorWebviewLocalStorageKeys } from '@/code_gen/typeshare/fluster_core_utilities'
import { getFakeNoteContent } from '@/utils/window_utils/development_window_utils/test_methods/set_test_content'
import React, { useEffect, type ReactNode } from 'react'


const HandleEditorState = (): ReactNode => {
    const testContent = getFakeNoteContent()
    const dispatch = useCodeEditorDispatch()
    useEffect(() => {
        dispatch({
            type: "setParsedEditorContentString",
            payload: testContent
        })
        window.localStorage.setItem("dark-mode", "true")
        window.localStorage.setItem(SplitviewEditorWebviewLocalStorageKeys.InitialValue, testContent)
    }, [testContent, dispatch])
    return null
}



export const DevelopmentTestPage = (): ReactNode => {
    return (
        <ResponsiveSplitViewEditor>
            <HandleEditorState />
        </ResponsiveSplitViewEditor>
    )
}


DevelopmentTestPage.displayName = "DevelopmentTestPage"
