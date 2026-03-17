import React, { type ReactNode } from 'react'
import { type EmbeddableComponentId, type InContentDocumentationFormat, type InContentDocumentationId } from '../../../core/code_gen/typeshare/fluster_core_utilities'


interface InContentDocumentationContainerProps {
    componentName?: EmbeddableComponentId
    inContentId?: InContentDocumentationId
    format: InContentDocumentationFormat
    children: ReactNode
}

export const InContentDocumentationContainer = ({ children }: InContentDocumentationContainerProps): ReactNode => {
    return (
        <div className="w-full border rounded p-4 bg-fd-card text-fd-card-foreground my-8 border-primary/40 inline-block">
            {children}
        </div>
    )
}


InContentDocumentationContainer.displayName = "InContentDocumentationContainer"
