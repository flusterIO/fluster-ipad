import React, { type ReactNode } from 'react'
import { type EmbeddableResponsiveGridPropsInput, embeddableResponsiveGridPropsSchema } from './embeddable_responsive_grid_props'
import { cn } from '@/utils/cn'
import { type WithChildren } from '@/utils/types/utility_types'

import { type WebviewContainerState } from '@/code_gen/typeshare/fluster_core_utilities';
import { connect } from 'react-redux';
import { type GlobalAppState } from '#/webview_global_state/store';
const connector = connect((state: GlobalAppState) => ({
    size: state.container.size,
}))

export const EmbeddableResponsiveGrid = connector(({ children, size, ...props }: EmbeddableResponsiveGridPropsInput & WithChildren & Pick<WebviewContainerState, "size">): ReactNode => {
    const { columns, containerClasses, responsiveTemplateColumns, responsive, emphasisClasses } = embeddableResponsiveGridPropsSchema.parse(props)


    const nColumns = columns[typeof size === "string" ? size : "none"]

    return (
        <div
            className={cn("grid w-full p-4 my-6", emphasisClasses, containerClasses)}
            style={{
                gridTemplateColumns: responsive ? responsiveTemplateColumns : typeof nColumns === "string" ? nColumns : `repeat(${nColumns}, 1fr)`
            }}
        >
            {children}
        </div>
    )
})


EmbeddableResponsiveGrid.displayName = "EmbeddableResponsiveGrid"
