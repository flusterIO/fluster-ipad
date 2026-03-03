import React, { HTMLProps, type ReactNode } from 'react'
import { EmbeddableUtilityContainerPropsInput, embeddableUtiltyContainerProps } from './embeddable_utility_container_props'
import { cn } from '@/utils/cn'
import { WithChildren } from '@/utils/types/utility_types'


export const EmbeddableUtilityContainer = ({ children, style, ...props }: EmbeddableUtilityContainerPropsInput & WithChildren & Pick<HTMLProps<HTMLDivElement>, "style">): ReactNode => {
    const { containerClasses, emphasisBackgroundClasses } = embeddableUtiltyContainerProps.parse(props)
    return (
        <div
            className={cn("w-full my-6", emphasisBackgroundClasses, containerClasses)}
            style={style}
        >
            {children}
        </div>
    )
}


EmbeddableUtilityContainer.displayName = "EmbeddableUtilityContainer"
