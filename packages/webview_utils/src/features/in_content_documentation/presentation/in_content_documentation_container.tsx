import React, { type ReactNode } from 'react'
import { EmbeddableComponentId } from '../../../core/code_gen/typeshare/fluster_core_utilities'


interface InContentDocumentationContainerProps {
    component: EmbeddableComponentId
}

export const InContentDocumentationContainer = ({ component }: InContentDocumentationContainerProps): ReactNode => {
    return (
        <div></div>
    )
}


InContentDocumentationContainer.displayName = "InContentDocumentationContainer"
