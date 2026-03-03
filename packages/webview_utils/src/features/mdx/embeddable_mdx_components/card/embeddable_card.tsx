import React, { type ReactNode } from 'react'
import { embeddableCardProps, EmbeddableCardPropsInput } from './embeddable_card_props'
import { WithChildren } from '@/utils/types/utility_types'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared_components/shad/card'
import { WithInlineMdx } from '#/mdx/components/inline_mdx_content'
import { cn } from '../../../../core/utils/cn'



export const EmbeddableCard = ({ children, InlineMdxContent, ...props }: EmbeddableCardPropsInput & WithChildren & WithInlineMdx): ReactNode => {
    const { title, desc, containerClasses, shrink, centerContent, centerBody } = embeddableCardProps.parse(props)
    return (
        <Card className={containerClasses} size={shrink ? "sm" : "default"}>
            <CardHeader>
                <CardTitle>
                    <InlineMdxContent
                        mdx={title}
                    />
                </CardTitle>
                {desc ? (
                    <CardDescription>
                        <InlineMdxContent
                            mdx={desc}
                        />
                    </CardDescription>
                ) : null}
            </CardHeader>
            <CardContent
                className={cn(centerContent ? "flex flex-col justify-center items-center" : undefined, centerBody && "w-full h-fit flex flex-col justify-center items-center")}
            >
                {children}
            </CardContent>
        </Card>
    )
}


EmbeddableCard.displayName = "EmbeddableCard"
