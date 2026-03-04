import React, { type ReactNode } from 'react'
import { EmbeddableComponentId, InContentDocumentationFormat, InContentDocumentationId } from '../../../core/code_gen/typeshare/fluster_core_utilities'


interface InContentDocumentationContainerProps {
    componentId?: EmbeddableComponentId
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
