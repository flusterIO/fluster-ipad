import React, { useEffect, type ReactNode } from 'react'
import { FlusterAiParsePendingContainer } from '../features/ai/presentation/ai_parse_pending_container'
import { CodeBlockParsingResult } from '@/code_gen/typeshare/fluster_core_utilities'



const testContent = `
# Test Content

<FlusterAIParsePending>
Summarize this note please.
</FlusterAIParsePending>
`



/**
 * Pass as a child to the ResponsiveSplitViewEditor to fake state accordigly.
 */
/* const FakeEditorStateHandler = (): ReactNode => { */
/*     const dispatch = useCodeEditorDispatch() */
/*     useEffect(() => { */
/*         const loadTestContent = async (): Promise<void> => { */
/*             const testContent = await getFakeNoteContent() */
/*             dispatch({ */
/*                 type: "setParsedEditorContentString", */
/*                 payload: testContent */
/*             }) */
/*             window.localStorage.setItem("dark-mode", "true") */
/*             window.localStorage.setItem(SplitviewEditorWebviewLocalStorageKeys.InitialValue, testContent) */
/*         } */
/*         loadTestContent() */
/*     }, [dispatch]) */
/*     return null */
/* } */



export const DevelopmentTestPage = (): ReactNode => {
    return (
        <div className="w-full h-full flex flex-col min-h-screen justify-center items-center">
            <FlusterAiParsePendingContainer stringifiedResult={JSON.stringify({
                block_content: "# Physics & Math \nCan you help me study?",
                full_match: "```fluster-ai\n# Physics & Math \nCan you help me study?",
                language_tag: "fluster-ai"
            } satisfies CodeBlockParsingResult)}>
            </FlusterAiParsePendingContainer>
        </div>
    )
}


DevelopmentTestPage.displayName = "DevelopmentTestPage"
