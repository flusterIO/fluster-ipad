import React, { type ReactNode } from 'react'
import { EmbeddableHintComponentPropsInput, embeddableHintComponentPropsSchema } from './hint_props_schema'
import { cn } from '../../../../core/utils/cn'


export const EmbeddableHintComponent = (props: EmbeddableHintComponentPropsInput): ReactNode => {
    const { label, children, containerClasses, textClasses } = embeddableHintComponentPropsSchema.parse(props)
    return (
        <div className={cn("text-sm mt-2 mb-6", containerClasses)}>
            <span className={cn("font-semibold pr-2", textClasses)}>{label}</span>
            <span className="[&>*]:inline! text-foreground/80 [&>*]:text-foreground/80 [&>*]:text-sm [&>p]:leading-tight text-sm leading-tight">
                {children}
            </span>
        </div>
    )
}


EmbeddableHintComponent.displayName = "EmbeddableHintComponent"
