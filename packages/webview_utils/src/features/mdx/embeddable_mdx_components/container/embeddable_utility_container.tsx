import React, { type ReactNode } from 'react'
import { EmbeddableUtilityContainerPropsInput, embeddableUtiltyContainerProps } from './embeddable_utility_container_props'
import { cn } from '@/utils/cn'
import { WithChildren } from '@/utils/types/utility_types'


export const EmbeddableUtilityContainer = ({ children, ...props }: EmbeddableUtilityContainerPropsInput & WithChildren): ReactNode => {
    const { containerClasses, emphasisBackgroundClasses } = embeddableUtiltyContainerProps.parse(props)
    return (
        <div
            className={cn("w-full", emphasisBackgroundClasses, containerClasses)}
        >
            {children}
        </div>
    )
}


EmbeddableUtilityContainer.displayName = "EmbeddableUtilityContainer"
