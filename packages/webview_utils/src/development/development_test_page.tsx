import { useCodeEditorDispatch } from '#/editor/code_editor/state/code_editor_provider'
import { MdxEditorPreviewOnly } from '#/mdx/components/mdx_content_preview_only'
import { getFakeNoteContent } from '@/utils/window_utils/development_window_utils/test_methods/set_test_content'
import React, { useEffect, type ReactNode } from 'react'




export const DevelopmentTestPage = (): ReactNode => {
    const testContent = getFakeNoteContent()
    const dispatch = useCodeEditorDispatch()
    useEffect(() => {
        dispatch({
            type: "setParsedEditorContentString",
            payload: testContent
        })
    }, [testContent])
    return (
        <MdxEditorPreviewOnly
        />
    )
}


DevelopmentTestPage.displayName = "DevelopmentTestPage"
