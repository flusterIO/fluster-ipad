import { AiParsePendingContainer } from '#/ai/presentation/ai_parse_pending_container'
import React, { useEffect, type ReactNode } from 'react'



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
            <AiParsePendingContainer >
                # AI

                Please help me generate a summary of this note.
            </AiParsePendingContainer>
        </div>
    )
}


DevelopmentTestPage.displayName = "DevelopmentTestPage"
