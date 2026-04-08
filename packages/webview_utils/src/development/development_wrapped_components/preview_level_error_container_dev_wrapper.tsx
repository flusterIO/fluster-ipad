import { type ConundrumError } from '@/code_gen/typeshare/conundrum'
import React, { useEffect, type ReactNode } from 'react'
import { faker } from "@faker-js/faker"
import { useDispatch } from 'react-redux'
import { appendConundrumError, setConundrumErrors } from '#/webview_global_state/conundrum_state/conundrum_slice'
import { PreviewLevelErrorReport } from '#/mdx/error_reporting/preview_level_error_report/preview_level_error_report'
import { MdxEditorPreview } from '#/mdx/components/mdx_editor_preview'
import { EditorStateActions } from '@/code_gen/typeshare/fluster_core_utilities'


const getFakeError = (): ConundrumError => {
    return {
        msg: faker.lorem.words({ min: 3, max: 20 }),
        details: Math.random() > 0.5 ? faker.lorem.paragraphs({ min: 1, max: 5 }).replaceAll("\n", "\n\n") : undefined,
        pos: undefined
    }
}

export const PreviewLevelErrorContainerDevWrapper = (): ReactNode => {
    const dispatch = useDispatch()
    useEffect(() => {
        dispatch(setConundrumErrors([getFakeError()]))
        window.handleSwiftAction({
            type: EditorStateActions.SetParsedEditorContent,
            payload: {
                value: "# My content"
            }
        })
    }, [])
    return (
        <MdxEditorPreview
        />
    )
}


PreviewLevelErrorContainerDevWrapper.displayName = "PreviewLevelErrorContainerDevWrapper"
