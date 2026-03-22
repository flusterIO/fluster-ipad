import React, { type ReactNode } from 'react'
import { FlusterAiParseButton } from "./ai_parse_button"
import { InlineMdxContent } from '#/mdx/components/inline_mdx_content'
import { type ParsedCodeBlock } from "../../../core/code_gen/typeshare/conundrum"


interface FlusterAiParsePendingContainerProps {
    /**
     * stringifiedResult is the stringified JSON of the CodeBlockParsingResult
     * that created the component so the data can be sent to swift when 'generate' is clicked 
     * with the current state of the component, not necessarily the current state of the note
     * to reflect the state that the user is actually seeing as the DB lags behind a bit.
     */
    res: ParsedCodeBlock | null
}

export const FlusterAiParsePendingContainer = ({ res }: FlusterAiParsePendingContainerProps): ReactNode => {
    if (!res) {
        console.error("Something went wrong while attempting to parse the json data passed into the FlusterAiParsePendingContainer.")
        return null
    }
    return (
        <div
            className="w-full max-w-[min(1080px,90%)] mx-auto p-4 border border-fd-ring bg-fd-card rounded text-card-foreground [&_*]:text-card-foreground @container/aiParsePendingContainer relative shadow-primary/50 border hover:border-primary transition-shadow duration-500"
        >
            <div
                className="absolute inset-x-0 top-0 h-px rounded-t-xl bg-gradient-to-r from-transparent via-primary/50 to-transparent"
                aria-hidden="true"
            />
            <div className="flex flex-col justify-start items-start">
                <div className="text-lg font-bold">AI Prompt:</div>
                <InlineMdxContent
                    mdx={res.content}
                    className="indent-4"
                />
            </div>
            <div className="w-full flex flex-row justify-end items-center">
                <div className="invisible @[540px]/aiParsePendingContainer:visible absolute bottom-3 left-3 text-sm opacity-60">
                    {`See 'AI?' docs for supported actions`}
                </div>
                <FlusterAiParseButton />
            </div>
        </div>
    )
}


FlusterAiParsePendingContainer.displayName = "FlusterAiParsePendingContainer"
