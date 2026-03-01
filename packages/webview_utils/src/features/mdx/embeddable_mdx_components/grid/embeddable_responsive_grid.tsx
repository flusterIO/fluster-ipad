import React, { type ReactNode } from 'react'
import { EmbeddableResponsiveGridPropsInput, embeddableResponsiveGridPropsSchema } from './embeddable_responsive_grid_props'
import { useWebviewContainerContext } from '#/webview_container/state/webview_provider'
import { cn } from '@/utils/cn'
import { WithChildren } from '@/utils/types/utility_types'



export const EmbeddableResponsiveGrid = ({ children, ...props }: EmbeddableResponsiveGridPropsInput & WithChildren): ReactNode => {
    const { size } = useWebviewContainerContext()
    const { columns, containerClasses } = embeddableResponsiveGridPropsSchema.parse(props)

    const nColumns = columns[typeof size === "string" ? size : "none"]

    return (
        <div
            className={cn("grid w-full p-4", containerClasses)}
            style={{
                gridTemplateColumns: typeof nColumns === "string" ? nColumns : `repeat(${nColumns}, 1fr)`
            }}
        >
            {children}
        </div>
    )
}


EmbeddableResponsiveGrid.displayName = "EmbeddableResponsiveGrid"
